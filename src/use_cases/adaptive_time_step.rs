use crate::domain::{
    output_config::OutputConfig, simulation_config::SimulationConfig, state::State,
};

pub struct TimeStep {
    pub delta: f64,
    pub is_frame_boundary: bool,
}

// Get the next time step, ensuring that we land on a frame boundary if it is within the base time step.
impl TimeStep {
    pub fn next(sim_config: &SimulationConfig, out_config: &OutputConfig, state: &State) -> Self {
        let min_speed = state
            .constraints
            .char_speed
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or_else(|| panic!("Characteristic speed is empty."))
            .clamp(0.1, 0.95); // Speed is in (0, 1], but don't go too slow or too fast.

        // This is a Courant number of 1, but adjusted for the speed of the slowest point,
        // so it should be stable.
        let base_time_step = sim_config.grid.delta * min_speed;

        // Find the next frame boundary after the current time
        let current_frame_index = (state.time / out_config.dt).floor();
        let next_frame = (current_frame_index + 1.0) * out_config.dt;
        let time_to_next_frame = next_frame - state.time;

        // The first condition handles floating point errors if we're exactly at a frame boundary.
        if time_to_next_frame <= 1e-12 || time_to_next_frame > base_time_step {
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
