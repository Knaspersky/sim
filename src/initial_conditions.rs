use ultraviolet::DVec3;
use rand::Rng;
use crate::cdm::Cdm;

pub fn generate_uniform_distribution(num_particles: usize, box_size: f64) -> Vec<Cdm> {
    let mut rng = rand::thread_rng();
    (0..num_particles)
        .map(|_| {
            let pos = DVec3::new(
                rng.gen::<f64>() * box_size,
                rng.gen::<f64>() * box_size,
                rng.gen::<f64>() * box_size,
            );
            let vel = DVec3::zero(); // Start with zero velocity
            let mass = 1.0; // Uniform mass for now
            Cdm::new(pos, vel, mass)
        })
        .collect()
}

// You might want to add a function to generate more realistic initial conditions based on redshift
pub fn generate_cosmological_initial_conditions(num_particles: usize, box_size: f64, initial_redshift: f64) -> Vec<Cdm> {
    // This is a placeholder. In a real simulation, you'd use a more sophisticated method
    // to generate initial conditions based on the power spectrum at the given redshift
    generate_uniform_distribution(num_particles, box_size)
}