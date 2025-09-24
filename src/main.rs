mod app;
mod domain;
mod use_cases;

use app::cli::Args;
use app::file_output::JsonlStateOutputCreator;
use std::fs::File;
use std::time::Instant;
use use_cases::simulate::simulate;

fn main() {
    let (config, state) = Args::parse_args();

    let file = File::create("results/simulation_output.jsonl").expect("Could not create file");
    let jsonl_output = JsonlStateOutputCreator::new(file);

    let start = Instant::now();
    let num_steps = simulate(&config, state, &jsonl_output);
    let duration = start.elapsed();

    println!("Evolution completed in: {:.2?}", duration);
    println!("Number of steps: {}", num_steps);
    println!("Time per step: {:.2?}", duration / num_steps as u32);
}
