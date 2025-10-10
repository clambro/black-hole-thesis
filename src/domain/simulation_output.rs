use serde::Serialize;

#[derive(Serialize)]
pub struct SimulationOutput {
    pub time_taken_seconds: f64,
    pub num_steps: i32,
    pub final_simulation_time: f64,
    pub black_hole_mass: Option<f64>,
}
