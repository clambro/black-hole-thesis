use crate::domain::state::State;
use crate::use_cases::time_step::rk4_step;
use crate::use_cases::ports::StateOutput;

pub fn simulate(state: &State, state_output: &dyn StateOutput) -> i32 {
    let time_step = state.courant * state.grid.delta / state.wave_speed;
    let num_steps = (state.total_time / time_step).ceil() as i32;

    state_output.save_state(&state, 0.0);

    for step in 1..=num_steps {
        let state = rk4_step(&state, time_step);

        // For the black hole stuff we want roughly 5 seconds of real time per unit of simulation time.
        // At 30fps, this means saving every 1/150 units of simulation time.
        if step % 5 == 0 {
            let current_time = step as f64 * time_step;
            state_output.save_state(&state, current_time);
        }
    }

    return num_steps;
}