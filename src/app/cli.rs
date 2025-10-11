//! Command-line interface for the black hole reflection simulation.

use crate::domain::grid::Grid;
use crate::domain::output_config::OutputConfig;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::simulation_inputs::SimulationInputs;
use crate::use_cases::state_builder::build_initial_state;
use clap::Parser;

/// Command-line arguments for the black hole reflection simulation.
#[derive(Parser, Clone, Debug)]
#[command(name = "black-hole-reflection")]
#[command(about = "A black hole reflection simulation.")]
pub struct Args {
    /// Level of spatial discretization for the simulation grid.
    #[arg(long, default_value = "13")]
    pub level_of_discretization: u32,

    /// Initial amplitude of the scalar field perturbation.
    #[arg(long, default_value = "30.0")]
    pub amplitude: f64,

    /// Time interval between output frames.
    /// At 30fps, this default is approximately 5 seconds of real time per unit of simulation time.
    #[arg(long, default_value = "0.0067")]
    pub output_dt: f64,

    /// Discretization level for output data.
    #[arg(long, default_value = "9")]
    pub output_dx_level: u32,

    /// Maximum simulation time before timeout.
    /// The simulation should end with BH formation, but this is here as a safety.
    #[arg(long, default_value = "15.0")]
    pub max_time: f64,
}

impl Args {
    /// Parse command-line arguments into simulation inputs.
    pub fn parse_args() -> SimulationInputs {
        let args = Args::parse();

        let sim_config = SimulationConfig {
            grid: Grid::from_level_of_discretization(args.level_of_discretization),
            initial_amplitude: args.amplitude,
            max_time: args.max_time,
        };
        let out_config = OutputConfig {
            dt: args.output_dt,
            dx_level: args.output_dx_level,
        };
        let initial_state = build_initial_state(&sim_config);

        let inputs = SimulationInputs {
            sim_config,
            out_config,
            initial_state,
        };
        inputs.validate().expect("Invalid inputs");

        inputs
    }
}
