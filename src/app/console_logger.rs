use crate::domain::simulation_output::SimulationOutput;
use crate::use_cases::ports::SimulationLogger;
use std::io::{self, Write};

pub struct ConsoleLogger;

impl ConsoleLogger {
    pub fn new() -> Self {
        Self
    }
}

impl SimulationLogger for ConsoleLogger {
    fn log_progress(&self, elapsed_seconds: f64, num_steps: i32, simulation_time: f64) {
        print!(
            "\rReal time: {:.2}s, Simulation time: {:.3}s, Steps: {}",
            elapsed_seconds, simulation_time, num_steps
        );
        io::stdout()
            .flush()
            .unwrap_or_else(|_| panic!("Could not flush stdout"));
    }

    fn log_timeout_warning(&self, max_time: f64) {
        println!(
            "WARNING: Simulation time exceeded the max time of {:.3}s without BH formation.",
            max_time
        );
    }

    fn log_final_results(&self, output: &SimulationOutput) {
        println!(
            "\nEvolution completed in: {:.2?} seconds",
            output.time_taken_seconds
        );
        println!("Number of steps: {}", output.num_steps);
        println!(
            "Time per step: {:.2?} milliseconds",
            output.time_taken_seconds / output.num_steps as f64 * 1000.0
        );
        match output.black_hole_mass {
            Some(mass) => println!("Black hole mass: {:.4?}", mass),
            None => println!("Black hole mass: None"),
        }
        println!(
            "Final simulation time: {:.4?}",
            output.final_simulation_time
        );
    }
}
