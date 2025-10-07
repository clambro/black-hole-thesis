use crate::domain::config::Config;
use crate::domain::simulation_output::SimulationOutput;
use crate::domain::state::State;
use crate::domain::state_output::StateOutput;
use crate::use_cases::adaptive_time_step::TimeStep;
use crate::use_cases::integration::rk4_step;
use crate::use_cases::ports::StateOutputCreator;

pub fn simulate(
    config: &Config,
    mut state: State,
    state_output_creator: &dyn StateOutputCreator,
) -> SimulationOutput {
    let mut num_steps = 0;
    let mut black_hole_mass: Option<f64> = None;

    state_output_creator.save_state(&StateOutput::from_state(&state, config));

    while black_hole_mass.is_none() {
        let time_step = TimeStep::next(&config, &state);
        state = rk4_step(&config, &state, time_step.delta);
        num_steps += 1;

        black_hole_mass = state.get_black_hole_mass();

        if time_step.is_frame_boundary {
            state_output_creator.save_state(&StateOutput::from_state(&state, config));
        }
        if state.time > config.total_time {
            break;
        }
    }

    // TODO: Saving the final state messes with the frame rate. It should be saved as
    // a separate metadata file.
    state_output_creator.save_state(&StateOutput::from_state(&state, config));

    return SimulationOutput {
        num_steps,
        final_time: state.time,
        black_hole_mass: black_hole_mass.unwrap_or(-1.0), // TODO: Remove this.
    };
}
