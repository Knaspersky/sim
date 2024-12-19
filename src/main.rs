mod cdm;
mod simulation;
mod octree;
mod physics;
mod initial_conditions;
mod visualization;

use simulation::Simulation;
use visualization::output_particle_positions;
use crate::visualization::output_to_hdf5;

fn main() {
    let n_particles = 10000;
    let box_size = 1000.0;
    let n_steps = 100;
    let dt = 0.01;
    let initial_redshift = 63.0;
    
    let mut sim = Simulation::new(n_particles, box_size, initial_redshift);
    
    for step in 0..n_steps {
        sim.update(dt);
        if step % 100 == 0 || step % 10 == 0 || step == 10 {
            println!("Step {}: Time = {}", step, sim.time);
            output_particle_positions(&sim);
            
            // Output to HDF5 file
            let file = format!("output_step_{}.hdf5", step);
            if let Err(e) = output_to_hdf5(&sim, &file) {
                eprintln!("Error writing HDF5 file: {}", e);
            }
        }
    }
}