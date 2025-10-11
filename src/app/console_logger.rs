//! Console logging for the black hole reflection simulation.

use crate::domain::simulation_output::SimulationOutput;
use crate::use_cases::ports::SimulationLogger;
use std::io::{self, Write};

/// Console-based logger for simulation progress and results.
pub struct ConsoleLogger;

impl ConsoleLogger {
    /// Create a new console logger instance.
    pub fn new() -> Self {
        Self
    }
}

impl SimulationLogger for ConsoleLogger {
    /// Log simulation progress information.
    fn log_progress(
        &self,
        amplitude: f64,
        elapsed_seconds: f64,
        num_steps: i32,
        simulation_time: f64,
    ) {
        print!(
            "\rAmplitude: {:.3}, Real time: {:.2}s, Simulation time: {:.3}, Steps: {}",
            amplitude, elapsed_seconds, simulation_time, num_steps
        );
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => eprintln!("Could not flush stdout: {}", e),
        };
    }

    /// Log a timeout warning when simulation exceeds maximum time.
    fn log_timeout_warning(&self, max_time: f64) {
        println!(
            "WARNING: Simulation time exceeded the max time of {:.3}s without BH formation.",
            max_time
        );
    }

    /// Log the final simulation results.
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
