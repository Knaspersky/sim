use ultraviolet::DVec3;
use crate::cdm::Cdm;

pub struct OctreeNode {
    center: DVec3,
    size: f64,
    total_mass: f64,
    center_of_mass: DVec3,
    children: Option<Box<[OctreeNode; 8]>>,
    particle: Option<Cdm>,
}

impl OctreeNode {
    pub fn new(center: DVec3, size: f64) -> Self {
        Self {
            center,
            size,
            total_mass: 0.0,
            center_of_mass: DVec3::zero(),
            children: None,
            particle: None,
        }
    }

    pub fn insert(&mut self, particle: Cdm) {
        if self.particle.is_none() && self.children.is_none() {
            self.particle = Some(particle);
            self.update_mass_and_com();
            return;
        };

        if self.children.is_none() {
            self.subdivide();
            if let Some(existing_particle) = self.particle.take() {
                self.insert(existing_particle);
            }
        };

        let octant = self.get_octant(particle.pos);
        if let Some(children) = &mut self.children {
            children[octant].insert(particle);
        }

        self.update_mass_and_com();
    }

    fn subdivide(&mut self) {
        let half_size = self.size / 2.0;
        let offsets = [
            DVec3::new(-1.0, -1.0, -1.0),
            DVec3::new(1.0, -1.0, -1.0),
            DVec3::new(-1.0, 1.0, -1.0),
            DVec3::new(1.0, 1.0, -1.0),
            DVec3::new(-1.0, -1.0, 1.0),
            DVec3::new(1.0, -1.0, 1.0),
            DVec3::new(-1.0, 1.0, 1.0),
            DVec3::new(1.0, 1.0, 1.0),
        ];

        self.children = Some(Box::new(offsets.map(|offset| {
            let new_center = self.center + offset * half_size * 0.5;
            OctreeNode::new(new_center, half_size)
        })))
    }

    fn get_octant(&self, pos: DVec3) -> usize {
        let diff = pos - self.center;
        ((diff.x >= 0.0) as usize) |
            (((diff.y >= 0.0) as usize) << 1) |
            (((diff.z >= 0.0) as usize) << 2)
    }

    fn update_mass_and_com(&mut self) {
        self.total_mass = 0.0;
        self.center_of_mass = DVec3::zero();

        if let Some(particle) = &self.particle {
            self.total_mass = particle.mass;
            self.center_of_mass = particle.pos;
        } else if let Some(children) = &self.children {
            for child in children.iter() {
                self.total_mass += child.total_mass;
                self.center_of_mass += child.center_of_mass * child.total_mass;
            }
            if self.total_mass > 0.0 {
                self.center_of_mass /= self.total_mass;
            }
        }
    }

    pub fn calculate_force(&self, particle: &Cdm, theta: f64) -> DVec3 {
        if self.particle.is_some() && self.particle.as_ref().unwrap().pos != particle.pos {
            return crate::physics::calculate_gravity(particle, self.particle.as_ref().unwrap());
        }
        
        let r = self.center_of_mass - particle.pos;
        let r_mag = r.mag();
        
        if self.size / r_mag > theta || self.children.is_none() {
            return crate::physics::calculate_gravity_with_com(particle, self.total_mass, self.center_of_mass);
        }
        
        let mut total_force = DVec3::zero();
        if let Some(children) = &self.children {
            for child in children.iter() {
                total_force += child.calculate_force(particle, theta);
            }
        }
        
        total_force
    }
}