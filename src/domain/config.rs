use crate::domain::grid::Grid;

/// Static configuration for the simulation.
pub struct Config {
    pub grid: Grid,
    pub initial_amplitude: f64,
    pub courant_number: f64,
    pub total_time: f64,
    pub output_dt: f64,
    pub output_dx_level: u32,
}
