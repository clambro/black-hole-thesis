use crate::domain::{config::Config, state::State};

pub struct TimeStep {
    pub delta: f64,
    pub is_frame_boundary: bool,
}

// Get the next time step, ensuring that we land on a frame boundary if it is within the base time step.
impl TimeStep {
    pub fn next(config: &Config, state: &State) -> Self {
        let base_time_step = config.courant_number * config.grid.delta / config.wave_speed;

        // Find the next frame boundary after the current time
        let current_frame_index = (state.time / config.output_dt).floor();
        let next_frame = (current_frame_index + 1.0) * config.output_dt;
        let time_to_next_frame = next_frame - state.time;

        // The first condition handles floating point errors if we're exactly at a frame boundary.
        if time_to_next_frame <= 1e-12 || time_to_next_frame > base_time_step {
            return TimeStep {
                delta: base_time_step,
                is_frame_boundary: false,
            };
        } else {
            return TimeStep {
                delta: time_to_next_frame,
                is_frame_boundary: true,
            };
        }
    }
}
