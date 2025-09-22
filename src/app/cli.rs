use clap::Parser;
use crate::domain::boundary_conditions::BoundaryCondition;
use crate::domain::grid::Grid;
use crate::domain::state::State;

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
    pub fn parse_state_from_args() -> State {
        let args = Args::parse();
        let grid = Grid::from_level_of_discretization(args.level_of_discretization);
        let state = State::from_args(
            grid,
            args.wave_speed,
            args.amplitude,
            args.left_bc,
            args.right_bc,
            args.courant,
            args.total_time,
        );
        return state;
    }
}
