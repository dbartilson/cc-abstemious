use std::io::Write;

extern crate nalgebra as na;

mod preprocess;
mod elements;
mod incident_wave;
mod influence_matrix;
mod solve;

fn main() {

    let (user_input, mesh, eqn_map) = preprocess::preprocess();
    
    print!(" Assembling surface BEM influence matrices...");
    std::io::stdout().flush().unwrap();
    let (h, g) = influence_matrix::get_surface_influence_matrices(&user_input, &mesh, &eqn_map);
    println!(" Complete!");

    let (phi_inc, phi_inc_fp) = incident_wave::get_incident_wave(&user_input, &mesh, &eqn_map);

    print!(" Solving system (direct LU)...");
    std::io::stdout().flush().unwrap();
    let (phi, vn) = solve::solve_lu(&user_input, &h, &g, &phi_inc, &eqn_map.len());
    println!(" Complete!");

    print!(" Post-processing...");
    std::io::stdout().flush().unwrap();
    let (m, l) = influence_matrix::get_field_influence_matrices(&user_input, &mesh, &eqn_map);
    let _phi_fp = solve::get_fp(&m, &l, &phi, &vn, &phi_inc_fp);
    println!(" Complete!");

    println!(" Exiting...");
}
