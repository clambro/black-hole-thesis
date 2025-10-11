//! Black hole reflection simulation.
//!
//! A numerical simulation of black hole formation in a confined space using
//! Einstein's equations in polar-areal gauge with a spherically symmetric
//! massless scalar field.

mod app;
mod domain;
mod use_cases;

use app::cli::Args;
use app::console_logger::ConsoleLogger;
use app::file_output::JsonlStateOutputCreator;
use use_cases::simulate::simulate;

/// Main entry point for the black hole reflection simulation.
fn main() {
    let inputs = Args::parse_args();

    let mut jsonl_output = JsonlStateOutputCreator::new(&inputs.sim_config);
    let logger = ConsoleLogger::new();

    simulate(inputs, &mut jsonl_output, &logger);
}
