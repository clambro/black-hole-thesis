use crate::domain::config::Config;
use crate::domain::grid::Grid;
use crate::domain::state::State;
use crate::use_cases::state_builder::build_initial_state;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(name = "black-hole-reflection")]
#[command(about = "A black hole reflection simulation.")]
pub struct Args {
    #[arg(long, default_value = "10")]
    pub level_of_discretization: u32,

    #[arg(long, default_value = "20.0")]
    pub amplitude: f64,

    #[arg(long, default_value = "0.5")]
    pub courant: f64,

    #[arg(long, default_value = "0.65")]
    pub total_time: f64,

    // At 30fps, this default is approximately 5 seconds of real time per unit of simulation time.
    #[arg(long, default_value = "0.0067")]
    pub output_dt: f64,

    #[arg(long, default_value = "8")]
    pub output_dx_level: u32,
}

impl Args {
    pub fn parse_args() -> (Config, State) {
        let args = Args::parse();
        args.validate();

        let config = args.build_config_from_args();
        let state = build_initial_state(&config);
        return (config, state);
    }

    fn validate(&self) {
        if self.level_of_discretization < 5 {
            panic!("Level of discretization must be greater than 5.");
        }
        let num_points = Grid::get_length_at_discretization(self.level_of_discretization) as f64;
        if self.output_dt * num_points < 1.0 {
            panic!(
                "Output dt is too short for the level of discretization. \
                 The output dt must be greater than 1 / 2^level_of_discretization."
            );
        }
        if self.output_dx_level > self.level_of_discretization {
            panic!(
                "Output dx level is greater than the level of discretization. \
                 The output dx level must be less than or equal to the level of discretization."
            );
        }
    }

    fn build_config_from_args(&self) -> Config {
        return Config {
            grid: Grid::from_level_of_discretization(self.level_of_discretization),
            initial_amplitude: self.amplitude,
            courant_number: self.courant,
            total_time: self.total_time,
            output_dt: self.output_dt,
            output_dx_level: self.output_dx_level,
        };
    }
}
