use crate::simulation::Simulation;
use hdf5::*;
use hdf5::{Datatype, Dataspace};
use hdf5::Extents::{Null, Simple};
use hdf5::types;
use hdf5::SimpleExtents;
use ndarray::{array, Array, Array1, Array2, ArrayBase, ArrayView, Ix2, IxDyn, OwnedRepr};

pub fn output_to_hdf5(simulation: &Simulation, filename: &str) -> Result<()> {
    println!("Saving: the \"{}\" snapshot file...\nt = {:.15}\nz = {:.15}",
             filename, simulation.time, simulation.redshift);

    let file = File::create(filename)?;
    
    // Create Header group
    let header_group = file.create_group("Header")?;

    // Write header attributes
    let entropy_flags: Vec<u32> = vec![0, 1035025064, 1066451923, 185542567, 1072015029, 6]; // temp

    let adim:[usize; 1] = [ 6 ];
    let entropy_flags2  = Array::from_vec(entropy_flags);
    
    header_group.new_attr::<u32>()
        .shape(adim)
        .create("Flag_Entropy_ICs")?
        .write(&entropy_flags2)?;
    header_group.new_attr::<f64>()
        .create("BoxSize")?
        .write_scalar(&simulation.box_size)?;
    header_group.new_attr::<i32>()
        .create("Flag_Cooling")?
        .write_scalar(&0)?;
    header_group.new_attr::<i32>()
        .create("Flag_Feedback")?
        .write_scalar(&0)?;
    header_group.new_attr::<i32>()
        .create("Flag_Metals")?
        .write_scalar(&0)?;
    header_group.new_attr::<i32>()
        .create("Flag_Sfr")?
        .write_scalar(&0)?;
    header_group.new_attr::<i32>()
        .create("Flag_StellarAge")?
        .write_scalar(&0)?;
    header_group.new_attr::<f64>()
        .create("HubbleParam")?
        .write_scalar(&simulation.h)?;
    header_group.new_attr::<f64>()
        .shape(adim)
        .create("MassTable")?
        .write(&[0; 6])?;
    header_group.new_attr::<i32>()
        .create("NumFilesPerSnapshot")?
        .write_scalar(&1)?;
    header_group.new_attr::<i32>()
        .shape(adim)
        .create("NumPart_ThisFile")?
        .write(&[0, simulation.particles.len() as i32, 0, 0, 0, 0])?;
    header_group.new_attr::<u32>()
        .shape(adim)
        .create("NumPart_Total")?
        .write(&[0, simulation.particles.len() as u32, 0, 0, 0, 0])?;
    header_group.new_attr::<u32>()
        .shape(adim)
        .create("NumPart_Total_HighWord")?
        .write(&[0; 6])?;
    header_group.new_attr::<f64>()
        .create("Omega0")?
        .write_scalar(&simulation.omega_m)?;
    header_group.new_attr::<f64>()
        .create("OmegaLambda")?
        .write_scalar(&simulation.omega_lambda)?;
    header_group.new_attr::<f64>()
        .create("Redshift")?
        .write_scalar(&simulation.redshift)?;
    header_group.new_attr::<f64>()
        .create("Time")?
        .write_scalar(&(1.0 / (1.0 + simulation.redshift)))?;

    //Create particle data group
    let particles_group = file.create_group("PartType1")?;

    //Writing out the particle positions
    let num_particles = simulation.particles.len();
    let mut dims= [num_particles, 3];
    
    let mut data: Vec<f32> = simulation.particles.iter()
        .flat_map(|p| [p.pos.x as f32, p.pos.y as f32, p.pos.z as f32])
        .collect();
    
    for value in data.iter_mut() {
        *value *= simulation.h as f32;
    }
    
    let mut array: ArrayBase<OwnedRepr<f32>, Ix2> = Array2::from_shape_vec(dims, data)?;

    particles_group.new_dataset::<f32>()
        .shape(dims)
        .create("Coordinates")?
        .write(&array)?;
    
    for value in array.iter_mut() {
        *value /= simulation.h as f32;
    }
    //particle positions written

    //Writing out the particle velocities
    let a = 1.0/(1.0 + simulation.redshift);
    const UNIT_V: f64 = 20.738652969925447;
    let scale = a.sqrt() * UNIT_V;
    let mut velocity_buf: Vec<f32> = Vec::with_capacity(3*num_particles);
    
    for particle in &simulation.particles {
        let scaled_vel = particle.vel * scale;
        velocity_buf.extend_from_slice(&[
            scaled_vel.x as f32,
            scaled_vel.y as f32,
            scaled_vel.z as f32
        ]);
    }
    
    array = Array::from_shape_vec(dims, velocity_buf)?;

    particles_group.new_dataset::<f32>()
        .shape(dims)
        .create("Velocities")?
        .write(&array)?;
    //Particle velocities written
    
    //Writing out particle IDs
    let particle_ids: Vec<u64> = (0..num_particles as u64).collect();
    let id_array = Array1::from_vec(particle_ids);
    particles_group.new_dataset::<u64>()
        .shape([num_particles])
        .create("ParticleIDs")?
        .write(&id_array)?;
    //particle IDs written
    
    //Writing particle Masses
    let masses: Vec<f64> = simulation.particles.iter().map(|p| p.mass).collect();

    let mut scaled_masses: Vec<f64> = masses.to_vec();

    for mass in &mut scaled_masses {
        *mass *= simulation.h;
    }
    
    let mass_array = Array1::from_vec(scaled_masses);
    particles_group.new_dataset::<f64>()
        .shape([num_particles])
        .create("Masses")?
        .write(&mass_array)?;
    //Masses written
    Ok(())
}

pub fn output_particle_positions(simulation: &Simulation) {
    println!("Time: {}", simulation.time);
    for (i, particle) in simulation.particles.iter().enumerate().take(10) {
        println!("Particle {}: pos = {:?}, vel = {:?}, acc = {:?}", i, particle.pos, particle.vel, particle.acc);
    }
    println!("... (showing first 10 particles)");
}