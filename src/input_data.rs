pub mod input_data {
    use serde::Deserialize;
    use std::fs::File;
    use std::io::BufReader;
    use std::error::Error;
    use std::path::Path;
    use std::io::Write;

    #[derive(Deserialize)]
    pub enum ProblemType {
        Interior,
        Exterior
    }    

    #[derive(Deserialize)]
    pub enum WaveType {
        PlaneWave,
        SphericalWave,
    }

    #[derive(Deserialize)]
    pub struct IncidentWaveInput {
        pub origin: [f64;3],
        pub wave_type: WaveType,
        pub amplitude: [f64;2]
    }

    #[derive(Deserialize)]
    pub enum BCType {
        Pressure,
        NormalVelocity,
        Impedance
    }

    #[derive(Deserialize)]
    pub struct SurfaceBoundaryCondition {
        pub bc_type: BCType,
        pub value: [f64; 2]
    }

    #[derive(Deserialize)]
    pub struct UserInput {
        pub mesh_file: String,
        pub body_index: usize,
        pub frequency: f64,
        pub sound_speed: f64,
        pub mass_density: f64,
        pub problem_type: ProblemType,
        pub field_points: Vec<[f64;3]>,
        pub incident_wave: IncidentWaveInput,
        pub surface_bc: SurfaceBoundaryCondition
    }

    pub fn read_input_json<P: AsRef<Path>>(path: P) -> Result<UserInput, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        print!(" Reading json file '{}' ...", path.as_ref().display().to_string());
        std::io::stdout().flush().unwrap();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
    
        // Read the JSON contents of the file as an instance of `User`.
        let u = serde_json::from_reader(reader)?;
        println!(" Complete!");
        // Return the `User`.
        Ok(u)
    }
}