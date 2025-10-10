use crate::domain::parsed_inputs::ParsedInputs;
use crate::domain::simulation_output::SimulationOutput;
use crate::use_cases::adaptive_time_step::TimeStep;
use crate::use_cases::integration::rk4_step;
use crate::use_cases::ports::{SimulationLogger, StateOutputCreator};
use std::time::Instant;

pub fn simulate(
    inputs: &ParsedInputs,
    state_output_creator: &dyn StateOutputCreator,
    logger: &dyn SimulationLogger,
) {
    let start = Instant::now();

    let mut num_steps: i32 = 0;
    let mut black_hole_mass = inputs.initial_state.get_black_hole_mass();
    let mut state = inputs.initial_state.clone();

    state_output_creator.save_state(&state, &inputs.out_config); // Initial state

    while black_hole_mass.is_none() {
        num_steps += 1;
        let time_step = TimeStep::next(&inputs.sim_config, &inputs.out_config, &state);

        state = rk4_step(&inputs.sim_config, &state, time_step.delta);
        black_hole_mass = state.get_black_hole_mass();

        if num_steps % 100 == 0 {
            let elapsed_seconds = start.elapsed().as_secs_f64();
            logger.log_progress(elapsed_seconds, num_steps, state.time);
        }

        if time_step.is_frame_boundary {
            state_output_creator.save_state(&state, &inputs.out_config);
        }
        if state.time > inputs.sim_config.max_time {
            logger.log_timeout_warning(inputs.sim_config.max_time);
            break;
        }
    }

    let output = SimulationOutput {
        time_taken_seconds: start.elapsed().as_secs_f64(),
        num_steps,
        final_simulation_time: state.time,
        black_hole_mass,
    };
    logger.log_final_results(&output);
    state_output_creator.save_final_results(&output);
}
