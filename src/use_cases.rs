//! Use cases layer for the black hole reflection simulation.
//!
//! This module contains the business logic and use cases that orchestrate
//! the simulation, including numerical methods, constraint solving, and
//! state management.

pub mod adaptive_time_step;
pub mod constraint_computer;
pub mod diff;
pub mod equations;
pub mod integration;
pub mod ports;
pub mod simulate;
pub mod state_builder;
pub mod state_output_builder;
