mod app;
mod domain;
mod use_cases;

use clap::Parser;
use domain::boundary_conditions::BoundaryCondition;
use domain::grid::Grid;
use domain::state::State;
use use_cases::time_step::rk4_step;

use std::fs::File;
use std::io::Write;
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
    let mut state = State::from_args(
        grid,
        args.wave_speed,
        args.amplitude,
        args.left_bc,
        args.right_bc,
    );

    let mut file = File::create("results/simulation_output.jsonl").expect("Could not create file");

    let time_step = args.courant * state.grid.delta / args.wave_speed;
    let num_steps = (args.total_time / time_step).ceil() as i32;

    // Save initial state
    save_state_to_jsonl(&mut file, &state, 0.0);

    let start = Instant::now();

    for step in 1..=num_steps {
        let next_state = rk4_step(&state, time_step);
        state = next_state;

        // For the black hole stuff we want roughly 5 seconds of real time per unit of simulation time.
        // At 30fps, this means saving every 1/150 units of simulation time.
        if step % 5 == 0 {
            let current_time = step as f64 * time_step;
            save_state_to_jsonl(&mut file, &state, current_time);
        }
    }

    let duration = start.elapsed();
    println!("Evolution completed in: {:.2?}", duration);
    println!("Time per step: {:.2?}", duration / num_steps as u32);
}

fn save_state_to_jsonl(file: &mut File, state: &State, time: f64) {
    let position_str = format!(
        "[{}]",
        state
            .wave_position
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    writeln!(file, "{{\"time\":{},\"position\":{}}}", time, position_str)
        .expect("Could not write to file");
}
