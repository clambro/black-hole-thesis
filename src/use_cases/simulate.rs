use crate::domain::constants::STEPS_PER_PROGRESS_UPDATE;
use crate::domain::simulation_inputs::SimulationInputs;
use crate::domain::simulation_output::SimulationOutput;
use crate::use_cases::adaptive_time_step::TimeStep;
use crate::use_cases::integration::rk4_step;
use crate::use_cases::ports::{SimulationLogger, StateOutputCreator};
use std::time::Instant;

/// Run the complete simulation until black hole formation or timeout.
pub fn simulate(
    inputs: SimulationInputs,
    state_output_creator: &mut dyn StateOutputCreator,
    logger: &dyn SimulationLogger,
) {
    let start = Instant::now();

    let mut num_steps: i32 = 0;
    let mut black_hole_mass = inputs.initial_state.black_hole_mass();
    let mut state = inputs.initial_state;

    if !inputs.out_config.skip_state_output {
        // Initial state
        state_output_creator.save_state(&state, &inputs.out_config, &inputs.sim_config);
    }

    while black_hole_mass.is_none() {
        num_steps += 1;
        let time_step = TimeStep::next(&inputs.sim_config, &inputs.out_config, &state);

        state = rk4_step(&inputs.sim_config, &state, time_step.delta);
        black_hole_mass = state.black_hole_mass();

        if num_steps % STEPS_PER_PROGRESS_UPDATE == 0 {
            let elapsed_seconds = start.elapsed().as_secs_f64();
            logger.log_progress(
                inputs.sim_config.initial_amplitude,
                elapsed_seconds,
                num_steps,
                state.time,
            );
        }

        if time_step.is_frame_boundary && !inputs.out_config.skip_state_output {
            state_output_creator.save_state(&state, &inputs.out_config, &inputs.sim_config);
        }
        if state.time > inputs.sim_config.max_time {
            logger.log_timeout_warning(inputs.sim_config.max_time);
            break;
        }
    }

    let output = SimulationOutput {
        initial_amplitude: inputs.sim_config.initial_amplitude,
        grid_level: inputs.sim_config.grid.level,
        time_taken_seconds: start.elapsed().as_secs_f64(),
        num_steps,
        final_simulation_time: state.time,
        black_hole_mass,
    };
    logger.log_final_results(&output);
    state_output_creator.save_final_results(&output);
}
