use crate::domain::boundary_conditions::BoundaryConditions;
use crate::domain::grid::Grid;

/// Static configuration for the simulation.
pub struct Config {
    pub grid: Grid,
    pub boundary_conditions: BoundaryConditions,
    pub wave_speed: f64,
    pub initial_amplitude: f64,
    pub courant_number: f64,
    pub total_time: f64,
}
