use crate::domain::state::State;
use crate::use_cases::time_step::rk4_step;
use std::fs::File;
use std::io::Write;

pub fn simulate(state: &State, courant: f64, total_time: f64) -> i32 {
    let time_step = courant * state.grid.delta / state.wave_speed;
    let num_steps = (total_time / time_step).ceil() as i32;

    let mut file = File::create("results/simulation_output.jsonl").expect("Could not create file");
    save_state_to_jsonl(&mut file, &state, 0.0);

    for step in 1..=num_steps {
        let state = rk4_step(&state, time_step);

        // For the black hole stuff we want roughly 5 seconds of real time per unit of simulation time.
        // At 30fps, this means saving every 1/150 units of simulation time.
        if step % 5 == 0 {
            let current_time = step as f64 * time_step;
            save_state_to_jsonl(&mut file, &state, current_time);
        }
    }

    return num_steps;
}

fn save_state_to_jsonl(file: &mut File, state: &State, time: f64) {
    let position_str = format!(
        "[{}]",
        state
            .wave_position
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    writeln!(file, "{{\"time\":{},\"position\":{}}}", time, position_str)
        .expect("Could not write to file");
}
