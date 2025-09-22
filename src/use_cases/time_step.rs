use crate::domain::state::State;
use crate::use_cases::equations::RightHandSide;
use crate::use_cases::vector_math::{vec_add, vec_scalar_mul};

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(state: &State, time_step: f64) -> State {
    let u1 = RightHandSide::new(
        &state.grid,
        state.wave_speed,
        &state.wave_position,
        &state.wave_velocity,
        &state.boundary_conditions,
    );
    let u2 = RightHandSide::new(
        &state.grid,
        state.wave_speed,
        &vec_add(
            &state.wave_position,
            &vec_scalar_mul(0.5 * time_step, &u1.position_dot),
        ),
        &vec_add(
            &state.wave_velocity,
            &vec_scalar_mul(0.5 * time_step, &u1.velocity_dot),
        ),
        &state.boundary_conditions,
    );
    let u3 = RightHandSide::new(
        &state.grid,
        state.wave_speed,
        &vec_add(
            &state.wave_position,
            &vec_scalar_mul(0.5 * time_step, &u2.position_dot),
        ),
        &vec_add(
            &state.wave_velocity,
            &vec_scalar_mul(0.5 * time_step, &u2.velocity_dot),
        ),
        &state.boundary_conditions,
    );
    let u4 = RightHandSide::new(
        &state.grid,
        state.wave_speed,
        &vec_add(
            &state.wave_position,
            &vec_scalar_mul(time_step, &u3.position_dot),
        ),
        &vec_add(
            &state.wave_velocity,
            &vec_scalar_mul(time_step, &u3.velocity_dot),
        ),
        &state.boundary_conditions,
    );
    let rk4: RightHandSide = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);
    return State {
        grid: state.grid.clone(),
        boundary_conditions: state.boundary_conditions.clone(),
        wave_speed: state.wave_speed,
        wave_position: vec_add(&state.wave_position, &rk4.position_dot),
        wave_velocity: vec_add(&state.wave_velocity, &rk4.velocity_dot),
    };
}
