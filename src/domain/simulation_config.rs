use crate::domain::grid::Grid;

/// Static configuration for the simulation.
pub struct SimulationConfig {
    pub grid: Grid,
    pub initial_amplitude: f64,
    pub max_time: f64,
}
