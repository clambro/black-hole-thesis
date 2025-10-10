mod app;
mod domain;
mod use_cases;

use app::cli::Args;
use app::file_output::JsonlStateOutputCreator;
use std::time::Instant;
use use_cases::simulate::simulate;

fn main() {
    let (config, state) = Args::parse_args();

    let jsonl_output = JsonlStateOutputCreator::new(&config);

    let start = Instant::now();
    let output = simulate(&config, state, &jsonl_output);
    let duration = start.elapsed();

    println!("Evolution completed in: {:.2?} seconds", duration);
    println!("Number of steps: {}", output.num_steps);
    println!("Time per step: {:.2?}", duration / output.num_steps as u32);
    match output.black_hole_mass {
        Some(mass) => println!("Black hole mass: {}", mass),
        None => println!("Black hole mass: None"),
    }
    println!("Final simulation time: {}", output.final_simulation_time);
}
