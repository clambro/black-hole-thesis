use crate::domain::grid::Grid;
use crate::domain::output_config::OutputConfig;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::simulation_inputs::SimulationInputs;
use crate::use_cases::state_builder::build_initial_state;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(name = "black-hole-reflection")]
#[command(about = "A black hole reflection simulation.")]
pub struct Args {
    #[arg(long, default_value = "13")]
    pub level_of_discretization: u32,

    #[arg(long, default_value = "30.0")]
    pub amplitude: f64,

    // At 30fps, this default is approximately 5 seconds of real time per unit of simulation time.
    #[arg(long, default_value = "0.0067")]
    pub output_dt: f64,

    #[arg(long, default_value = "9")]
    pub output_dx_level: u32,

    // The simulation should end with BH formation, but this is here as a safety.
    #[arg(long, default_value = "15.0")]
    pub max_time: f64,
}

impl Args {
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
