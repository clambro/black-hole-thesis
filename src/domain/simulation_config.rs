use crate::domain::grid::Grid;

/// Static configuration for the simulation.
pub struct SimulationConfig {
    /// Spatial grid configuration.
    pub grid: Grid,
    /// Initial amplitude of the scalar field.
    pub initial_amplitude: f64,
    /// Maximum simulation time.
    pub max_time: f64,
}
