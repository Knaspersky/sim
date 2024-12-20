
use crate::simulation::Simulation;
use hdf5::*;
use hdf5::{Datatype, Dataspace};
use hdf5::Extents::{Null, Simple};
use hdf5::types;
use hdf5::SimpleExtents;
use ndarray::{array, Array, Array1, Array2, ArrayBase, ArrayView, Ix2, IxDyn, OwnedRepr};

//pub fn write_header_attributes_in_hdf5(hdf5::)

pub fn output_to_hdf5(simulation: &Simulation, filename: &str) -> Result<()> {
    println!("Saving: the \"{}\" snapshot file...\nt = {:.15}\nz = {:.15}",
             filename, simulation.time, simulation.redshift);

    let file = File::create(filename)?;
    let entropy_flags: Vec<u32> = vec![0, 1035025064, 1066451923, 185542567, 1072015029, 6];

    let adim:[usize; 1] = [ 6 ];
    let hdf5_dataspace = Dataspace::try_new(Simple(SimpleExtents::from(Extent::fixed(1))));

    let entropy_flags2  = Array::from_vec(entropy_flags);
    

  //println!("{:?}", &hdf5_dataspace?.as_attr()?.write(entropy_flags2));
    // Create Header group
    let header_group = file.create_group("Header")?;
  
    header_group.new_attr::<u32>().shape(adim).create("Flag_Entropy_ICs")?.write(&entropy_flags2)?;
        // Write header attributes
        header_group.new_attr::<f64>().create("BoxSize")?
        .write_scalar(&simulation.box_size)?;
    header_group.new_attr::<i32>().create("Flag_Cooling")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Feedback")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Metals")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Sfr")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_StellarAge")?.write_scalar(&0)?;
    header_group.new_attr::<f64>().create("HubbleParam")?.write_scalar(&simulation.h)?;
    header_group.new_attr::<f64>().shape(adim).create("MassTable")?.write(&[0; 6])?;
    header_group.new_attr::<i32>().create("NumFilesPerSnapshot")?.write_scalar(&1)?;
    header_group.new_attr::<i32>().shape(adim).create("NumPart_ThisFile")?.write(&[0, simulation.particles.len() as i32, 0, 0, 0, 0])?;
    header_group.new_attr::<u32>().shape(adim).create("NumPart_Total")?.write(&[0, simulation.particles.len() as u32, 0, 0, 0, 0])?;
    header_group.new_attr::<u32>().shape(adim).create("NumPart_Total_HighWord")?.write(&[0; 6])?;
    header_group.new_attr::<f64>().create("Omega0")?.write_scalar(&simulation.omega_m)?;
    header_group.new_attr::<f64>().create("OmegaLambda")?.write_scalar(&simulation.omega_lambda)?;
    header_group.new_attr::<f64>().create("Redshift")?.write_scalar(&simulation.redshift)?;
    header_group.new_attr::<f64>().create("Time")?.write_scalar(&(1.0 / (1.0 + simulation.redshift)))?;

    let particles_group = file.create_group("PartType1")?;

    //Writing out particle positions
    //let hdf5_dataspace = Dataspace::try_new(Simple(SimpleExtents::from(Extent::fixed(2))));
    let num_particles = simulation.particles.len();
    let dims= [num_particles, 3];
    let mut data: Vec<f32> = simulation.particles.iter()
        .flat_map(|p| [p.pos.x as f32, p.pos.y as f32, p.pos.z as f32])
        .collect();
    let mut array: ArrayBase<OwnedRepr<f32>, Ix2> = Array2::from_shape_vec(dims, data)?;
    
    let dataset = particles_group.new_dataset::<f32>().shape(dims).create("Coordinates")?;
    dataset.write(&array)?;
    
    //Writing out 
    /*
    header_group.new_attr::<i32>().create("Flag_Cooling")?.write_scalar(&0)?;
    header_group.new_attr::<[u32; 6]>().create("Flag_Entropy_ICs")?.write(&[0; 6])?;
    header_group.new_attr::<i32>().create("Flag_Feedback")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Metals")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Sfr")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_StellarAge")?.write_scalar(&0)?;
    header_group.new_attr::<f64>().create("HubbleParam")?.write_scalar(&simulation.h)?;
    header_group.new_attr::<[f64; 6]>().create("MassTable")?.write(&[0.0; 6])?;
    header_group.new_attr::<i32>().create("NumFilesPerSnapshot")?.write_scalar(&1)?;
    header_group.new_attr::<[i32; 6]>().create("NumPart_ThisFile")?.write(&[0, simulation.particles.len() as i32, 0, 0, 0, 0])?;
    header_group.new_attr::<[u32; 6]>().create("NumPart_Total")?.write(&[0, simulation.particles.len() as u32, 0, 0, 0, 0])?;
    header_group.new_attr::<[u32; 6]>().create("NumPart_Total_HighWord")?.write(&[0; 6])?;
    header_group.new_attr::<f64>().create("Omega0")?.write_scalar(&simulation.omega_m)?;
    header_group.new_attr::<f64>().create("OmegaLambda")?.write_scalar(&simulation.omega_lambda)?;
    header_group.new_attr::<f64>().create("Redshift")?.write_scalar(&simulation.redshift)?;
    header_group.new_attr::<f64>().create("Time")?.write_scalar(&(1.0 / (1.0 + simulation.redshift)))?;
     */
    // Create Header attributes
    /*header_group
        .new_attr::<f64>()
        .create("BoxSize")?
        .write_scalar(&simulation.box_size)?;
    header_group.new_attr::<i32>()
        .create("Flag_Cooling")?
        .write_scalar(&0)?;
//    header_group.new_attr::<[u32; 6]>()
  //      .create("Flag_Entropy_ICs")?
   //     .write(&[0; 6])?;
        //.write(&entropy_flags)?;
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
    header_group.new_attr::<[f64; 6]>()
        .shape([6])
        .create("MassTable")?
        .write(&[0; 6])?;

     */



    /* Write header attributes
    header_group.new_attr::<[u32; 6]>().shape([6]).create("NumPart_ThisFile")?.write(&[0, num_particles, 0, 0, 0, 0])?;
    header_group.new_attr::<[u32; 6]>().shape([6]).create("NumPart_Total")?.write(&[0, num_particles, 0, 0, 0, 0])?;
    header_group.new_attr::<[u32; 6]>().shape([6]).create("NumPart_Total_HighWord")?.write(&[0, 0, 0, 0, 0, 0])?;
    header_group.new_attr::<[f64; 6]>().shape([6]).create("MassTable")?.write(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?;
    header_group.new_attr::<f64>().create("Time")?.write_scalar(&simulation.time)?;
    header_group.new_attr::<f64>().create("Redshift")?.write_scalar(&simulation.redshift)?;
    header_group.new_attr::<f64>().create("BoxSize")?.write_scalar(&simulation.box_size)?;
    header_group.new_attr::<i32>().create("NumFilesPerSnapshot")?.write_scalar(&1)?;
    header_group.new_attr::<f64>().create("Omega0")?.write_scalar(&simulation.omega_m)?;
    header_group.new_attr::<f64>().create("OmegaLambda")?.write_scalar(&simulation.omega_lambda)?;
    header_group.new_attr::<f64>().create("HubbleParam")?.write_scalar(&simulation.h)?;
    header_group.new_attr::<i32>().create("Flag_Sfr")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Cooling")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_StellarAge")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Metals")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_Feedback")?.write_scalar(&0)?;
    header_group.new_attr::<i32>().create("Flag_DoublePrecision")?.write_scalar(&1)?;

    // Prepare and write particle data
    let positions: Vec<[f64; 3]> = simulation.particles.iter()
        .map(|p| [p.pos.x, p.pos.y, p.pos.z])
        .collect();

    let velocities: Vec<[f64; 3]> = simulation.particles.iter()
        .map(|p| [p.vel.x, p.vel.y, p.vel.z])
        .collect();

    let masses: Vec<f64> = simulation.particles.iter().map(|p| p.mass).collect();
    let particle_ids: Vec<u64> = (0..num_particles as u64).collect();

    particles_group.new_dataset::<[f64; 3]>().create("Coordinates")?.write(&positions)?;
    particles_group.new_dataset::<[f64; 3]>().create("Velocities")?.write(&velocities)?;
    particles_group.new_dataset::<f64>().create("Masses")?.write(&masses)?;
    particles_group.new_dataset::<u64>().create("ParticleIDs")?.write(&particle_ids)?;

    println!("HDF5 file '{}' created successfully!", filename);
   */
    Ok(())
}

pub fn output_particle_positions(simulation: &Simulation) {
    println!("Time: {}", simulation.time);
    for (i, particle) in simulation.particles.iter().enumerate().take(10) {
        println!("Particle {}: pos = {:?}, vel = {:?}", i, particle.pos, particle.vel);
    }
    println!("... (showing first 10 particles)");
}