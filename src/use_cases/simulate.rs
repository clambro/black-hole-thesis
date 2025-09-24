use crate::domain::config::Config;
use crate::domain::state::State;
use crate::domain::state_output::StateOutput;
use crate::use_cases::adaptive_time_step::TimeStep;
use crate::use_cases::integration::rk4_step;
use crate::use_cases::ports::StateOutputCreator;

pub fn simulate(
    config: &Config,
    mut state: State,
    state_output_creator: &dyn StateOutputCreator,
) -> i32 {
    let mut num_steps = 0;

    state_output_creator.save_state(&StateOutput::from_state(&state, config));

    while state.time < config.total_time {
        let time_step = TimeStep::next(&config, &state);
        state = rk4_step(&config, &state, time_step.delta);
        num_steps += 1;

        // For the black hole stuff we want roughly 5 seconds of real time per unit of simulation time.
        // At 30fps, this means saving every 1/150 units of simulation time.
        if time_step.is_frame_boundary {
            state_output_creator.save_state(&StateOutput::from_state(&state, config));
        }
    }

    return num_steps;
}
