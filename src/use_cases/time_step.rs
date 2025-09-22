use crate::domain::config::Config;
use crate::domain::state::State;
use crate::use_cases::equations::EquationsOfMotion;
use crate::use_cases::vector_math::{vec_add, vec_scalar_mul};

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(
        &config,
        &state.wave_position,
        &state.wave_velocity,
    );
    let u2 = EquationsOfMotion::new(
        &config,
        &vec_add(
            &state.wave_position,
            &vec_scalar_mul(0.5 * time_step, &u1.position_dot),
        ),
        &vec_add(
            &state.wave_velocity,
            &vec_scalar_mul(0.5 * time_step, &u1.velocity_dot),
        ),
    );
    let u3 = EquationsOfMotion::new(
        &config,
        &vec_add(
            &state.wave_position,
            &vec_scalar_mul(0.5 * time_step, &u2.position_dot),
        ),
        &vec_add(
            &state.wave_velocity,
            &vec_scalar_mul(0.5 * time_step, &u2.velocity_dot),
        ),
    );
    let u4 = EquationsOfMotion::new(
        &config,
        &vec_add(
            &state.wave_position,
            &vec_scalar_mul(time_step, &u3.position_dot),
        ),
        &vec_add(
            &state.wave_velocity,
            &vec_scalar_mul(time_step, &u3.velocity_dot),
        ),
    );
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);
    return State {
        wave_position: vec_add(&state.wave_position, &rk4.position_dot),
        wave_velocity: vec_add(&state.wave_velocity, &rk4.velocity_dot),
        time: state.time + time_step,
    };
}
