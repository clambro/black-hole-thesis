use crate::domain::{
    constants::{EPS, MAX_COURANT_NUMBER, MIN_COURANT_NUMBER},
    output_config::OutputConfig,
    simulation_config::SimulationConfig,
    state::State,
};

/// Adaptive time step with frame boundary detection.
pub struct TimeStep {
    /// Time step size.
    pub delta: f64,
    /// Whether this step lands on a frame boundary.
    pub is_frame_boundary: bool,
}

impl TimeStep {
    // Get the next time step, ensuring that we land on a frame boundary if it is within the base time step.
    pub fn next(sim_config: &SimulationConfig, out_config: &OutputConfig, state: &State) -> Self {
        let min_speed = state
            .constraints
            .char_speed
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .expect("Characteristic speed is empty.")
            .clamp(MIN_COURANT_NUMBER, MAX_COURANT_NUMBER);

        // This is a Courant number of up to MAX_COURANT_NUMBER, adjusted for the speed of the slowest point.
        let base_time_step = sim_config.grid.delta * min_speed;

        // Frame boundaries don't matter if we're skipping the state output.
        if out_config.skip_state_output {
            return TimeStep {
                delta: base_time_step,
                is_frame_boundary: false,
            };
        }

        // Find the next frame boundary after the current time
        let current_frame_index = (state.time / out_config.dt).floor();
        let next_frame = (current_frame_index + 1.0) * out_config.dt;
        let time_to_next_frame = next_frame - state.time;

        // The first condition handles floating point errors if we're exactly at a frame boundary.
        if time_to_next_frame <= EPS || time_to_next_frame > base_time_step {
            return TimeStep {
                delta: base_time_step,
                is_frame_boundary: false,
            };
        }
        TimeStep {
            delta: time_to_next_frame,
            is_frame_boundary: true,
        }
    }
}
