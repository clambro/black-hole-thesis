use serde::Serialize;

/// Final results of a simulation run.
#[derive(Serialize)]
pub struct SimulationOutput {
    /// The initial amplitude of the wave.
    pub initial_amplitude: f64,
    /// The level of the grid.
    pub grid_level: u32,
    /// Wall-clock time taken for the simulation.
    pub time_taken_seconds: f64,
    /// Number of time steps completed.
    pub num_steps: i32,
    /// Final simulation time reached.
    pub final_simulation_time: f64,
    /// Black hole mass if one formed.
    pub black_hole_mass: Option<f64>,
}
