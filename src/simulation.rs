use rayon::prelude::*;
use ultraviolet::DVec3;
use crate::cdm::Cdm;
use crate::physics::{calculate_gravity, MIN_DISTANCE};
use crate::initial_conditions::generate_uniform_distribution;
use crate::octree::OctreeNode;

pub struct Simulation {
    pub particles: Vec<Cdm>,
    pub box_size: f64,
    pub time: f64,
    pub(crate) redshift: f64,
    pub theta: f64,
    pub initial_redshift: f64,
    pub omega_m: f64,
    pub omega_lambda: f64,
    pub h: f64,
}

// const DT: f64 = 0.01;
// const MIN: f64 = 0.01;

impl Simulation {
    pub fn new(n_particles: usize, box_size: f64, initial_redshift: f64) -> Self {
        let particles = generate_uniform_distribution(n_particles, box_size);
        Self {
            particles,
            box_size,
            time: 0.0,
            redshift: initial_redshift,
            theta: 0.5,
            initial_redshift,
            omega_m: 0.3111,
            omega_lambda: 0.6889,
            h: 0.6766,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let octree = self.build_octree();
        self.calculate_acceleration(&octree);
        for particle in &mut self.particles {
            particle.update(dt);
        }
        self.apply_periodic_boundary();
        self.time += dt;
        self.update_redshift(dt);
    }
    
    fn update_redshift(&mut self, dt: f64) {
        let a = 1.0 / (1.0 + self.redshift);
        let h0 = self.h * 100.0 / 20.738652969925447;;
        let da_dt = h0 * a *(self.omega_m * a.powi(-3) + self.omega_lambda).sqrt();
        let new_a = a + da_dt * dt;
        self.redshift = 1.0 / new_a - 1.0;
    }

    fn build_octree(&self) -> OctreeNode {
        let mut root = OctreeNode::new(DVec3::new(self.box_size / 2.0, self.box_size / 2.0, self.box_size / 2.0), self.box_size);
        for particle in &self.particles {
            root.insert(particle.clone());
        }
        root
    }
    
    fn calculate_acceleration(&mut self, octree: &OctreeNode) {
        self.particles.par_iter_mut().for_each(|particle| {
            particle.acc = octree.calculate_force(particle, self.theta) / particle.mass;
        });
    }
    
    fn apply_periodic_boundary(&mut self) {
        for particle in &mut self.particles {
            particle.pos.x = particle.pos.x.rem_euclid(self.box_size);
            particle.pos.y = particle.pos.y.rem_euclid(self.box_size);
            particle.pos.z = particle.pos.z.rem_euclid(self.box_size);
        }
    }
    
/*    pub fn update(&mut self) {
        self.calculate_acc();
        for i in 0..self.particles.len() {
            self.particles[i].update(DT)
        }
        self.apply_periodic_boundary();
        self.time += DT;
    }*/

/*    fn calculate_acc(&mut self) {
        for i in 0..self.particles.len() {
            let p1 = self.particles[i].pos;
            let m1 = self.particles[i].mass;
            for j in (i + 1)..self.particles.len() {
                if j != i {
                    let p2 = self.particles[j].pos;
                    let m2 = self.particles[j].mass;

                    let r = p2 - p1;
                    let mag_sq = r.x*r.x + r.y*r.y + r.z*r.z;
                    let mag = mag_sq.sqrt();
                    let tmp = r / (mag_sq.max(MIN) * mag);

                    self.particles[i].acc += m2 * tmp;
                    self.particles[j].acc -= m1 * tmp;
                }
            }
        }
    }*/

  

/*    fn generate_initial_conditions(n_particles: usize, box_size: f64) -> Vec<Cdm> {
        let mut rng = rand::thread_rng();
        (0..n_particles)
            .map(|_| {
                let pos = DVec3::new(
                    rng.gen::<f64>() * box_size,
                    rng.gen::<f64>() * box_size,
                    rng.gen::<f64>() * box_size,
                );
                let vel = DVec3::zero(); // Start with zero velocity for now
                let mass = 1.0; // Uniform mass for now
                Cdm::new(pos,vel, mass)
        })
            .collect()
    }*/

}