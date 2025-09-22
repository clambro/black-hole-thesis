mod app;
mod domain;
mod use_cases;

use clap::Parser;
use domain::boundary_conditions::BoundaryCondition;
use domain::grid::Grid;
use domain::state::State;
use use_cases::simulate::simulate;

use std::fs::File;
use std::time::Instant;

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

    #[arg(long, default_value = "10.0")]
    pub total_time: f64,

    #[arg(long, default_value = "dirichlet")]
    pub left_bc: BoundaryCondition,

    #[arg(long, default_value = "neumann")]
    pub right_bc: BoundaryCondition,
}

fn main() {
    let args = Args::parse();
    let grid = Grid::from_level_of_discretization(args.level_of_discretization);
    let state = State::from_args(
        grid,
        args.wave_speed,
        args.amplitude,
        args.left_bc,
        args.right_bc,
    );

    let start = Instant::now();
    let num_steps = simulate(&state, args.courant, args.total_time);
    let duration = start.elapsed();
    
    println!("Evolution completed in: {:.2?}", duration);
    println!("Time per step: {:.2?}", duration / num_steps as u32);
}

