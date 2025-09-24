use crate::domain::config::Config;
use crate::domain::state::State;
use crate::use_cases::ports::StateOutput;
use crate::use_cases::time::rk4_step;

pub fn simulate(config: &Config, mut state: State, state_output: &dyn StateOutput) -> i32 {
    let time_step = config.courant_number * config.grid.delta / config.wave_speed;
    let num_steps = (config.total_time / time_step).ceil() as i32;

    state_output.save_state(&state);

    for step in 1..=num_steps {
        state = rk4_step(&config, &state, time_step);

        // For the black hole stuff we want roughly 5 seconds of real time per unit of simulation time.
        // At 30fps, this means saving every 1/150 units of simulation time.
        if step % 5 == 0 {
            state_output.save_state(&state);
        }
    }

    return num_steps;
}
