use std::f64::consts::PI;

use cc_abstemious::preprocess::input_data::*;

fn default_input() -> UserInput {
    UserInput {
        mesh_file: "./src/tests/sphere.vtk".to_string(),
        body_index: 3,
        frequency: Vec::new(),
        sound_speed: 1.0,
        mass_density: 1.0,
        problem_type: ProblemType::Exterior,

        incident_wave: IncidentWaveInput {
            origin: [0.0, 0.0, 0.0],
            wave_type: WaveType::SphericalWave,
            amplitude: [0.0, 0.0]
        },
        surface_bc: SurfaceBoundaryCondition {
            bc_type: BCType::NormalVelocity,
            value: [0.0, 0.0]
        },
        output: Output {
            o_type: OutputType::Scattered,
            field: OutputField::Pressure,
            field_points: Vec::new(),
            file: "".to_string()
        }
    }
}

fn linspace(start: f64, end: f64, npts: usize) -> Vec<f64> {
    let dx = (end - start) / ((npts - 1) as f64);
    let mut x = vec![start; npts];
    for i in 1..(npts-1) {
        x[i] = start + dx * (i as f64);
    }
    x[npts-1] = end;
    return x;
}

fn logspace(start: f64, end: f64, npts: usize) -> Vec<f64> {

    let start_log = start.log10();
    let end_log = end.log10();
    let x_log = linspace(start_log, end_log, npts);
    let mut x = vec![start; npts];
    x[npts-1] = end;
    for i in 1..(npts-1) {
        x[i] = 10.0_f64.powf(x_log[i]);
    }
    return x;
}

#[test]
fn rigid_sphere_plane_wave_ring() {
    let mut analysis = cc_abstemious::Analysis::new();
    let mut input = default_input();
    input.frequency = vec![10.0];
    // water
    input.sound_speed = 1500.0;
    input.mass_density = 1000.0;
    // incident wave
    input.incident_wave.origin = [1.0, 0.0, 0.0];
    input.incident_wave.wave_type = WaveType::PlaneWave;
    input.incident_wave.amplitude = [1.0, 0.0];
    // output fule
    input.output.file = "./src/tests/rigid_sphere_plane_wave_bem.csv".to_string();
    // set up field points (ring in XY plane)
    let num_fp = 100;
    let radius = 10.0;
    for i in 0..num_fp {
        let theta = 2.0 * PI * (i as f64) / (num_fp as f64);
        let x = radius * f64::cos(theta);
        let y = radius * f64::sin(theta);
        input.output.field_points.push([x, y, 0.0]);
    }

    analysis.set_input(input);
    analysis.run();
    analysis.write_results_at_frequency(0);
}

#[test]
fn rigid_sphere_plane_wave_sweep() {
    let mut analysis = cc_abstemious::Analysis::new();
    let mut input = default_input();
    input.frequency = linspace(10.0, 1000.0, 50);
    // water
    input.sound_speed = 1500.0;
    input.mass_density = 1000.0;
    // incident wave
    input.incident_wave.origin = [1.0, 0.0, 0.0];
    input.incident_wave.wave_type = WaveType::PlaneWave;
    input.incident_wave.amplitude = [1.0, 0.0];
    // output fule
    input.output.file = "./src/tests/rigid_sphere_plane_wave_bem.csv".to_string();
    // set up field points (ring in XY plane)
    let radius = 10.0;
    let theta = 0.0;
    let x = radius * f64::cos(theta);
    let y = radius * f64::sin(theta);
    input.output.field_points.push([x, y, 0.0]);

    analysis.set_input(input);
    analysis.run();
    analysis.write_results_at_point(0);
    let _fp = analysis.get_result();
}