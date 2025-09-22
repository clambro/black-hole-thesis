use crate::domain::boundary_conditions::{BoundaryCondition, BoundaryConditions};
use crate::domain::config::Config;
use crate::domain::grid::Grid;
use crate::domain::state::State;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(name = "black-hole-reflection")]
#[command(about = "A black hole reflection simulation.")]
pub struct Args {
    #[arg(long, default_value = "8")]
    pub level_of_discretization: u32,

    #[arg(long, default_value = "1.0")]
    pub wave_speed: f64,

    #[arg(long, default_value = "1.0")]
    pub amplitude: f64,

    #[arg(long, default_value = "0.5")]
    pub courant: f64,

    #[arg(long, default_value = "5.0")]
    pub total_time: f64,

    #[arg(long, default_value = "dirichlet")]
    pub left_bc: BoundaryCondition,

    #[arg(long, default_value = "neumann")]
    pub right_bc: BoundaryCondition,
}

impl Args {
    pub fn parse_args() -> (Config, State) {
        let args = Args::parse();

        let config = args.build_config_from_args();
        let state = args.build_state_from_args(&config);
        return (config, state);
    }

    fn build_config_from_args(&self) -> Config {
        return Config {
            grid: Grid::from_level_of_discretization(self.level_of_discretization),
            boundary_conditions: BoundaryConditions {
                left: self.left_bc.clone(),
                right: self.right_bc.clone(),
            },
            wave_speed: self.wave_speed,
            initial_amplitude: self.amplitude,
            courant_number: self.courant,
            total_time: self.total_time,
        };
    }

    fn build_state_from_args(&self, config: &Config) -> State {
        return State {
            time: 0.0,
            displacement: State::get_initial_displacement(&config.grid, config.initial_amplitude),
            momentum: State::get_initial_momentum(&config.grid),
        };
    }
}
