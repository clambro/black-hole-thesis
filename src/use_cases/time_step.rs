use crate::domain::config::Config;
use crate::domain::state::State;
use crate::use_cases::equations::EquationsOfMotion;
use crate::use_cases::vector_math::{vec_add, vec_scalar_mul};

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(
        &config,
        &state.displacement,
        &state.momentum,
    );
    let u2 = EquationsOfMotion::new(
        &config,
        &vec_add(
            &state.displacement,
            &vec_scalar_mul(0.5 * time_step, &u1.d_dt_displacement),
        ),
        &vec_add(
            &state.momentum,
            &vec_scalar_mul(0.5 * time_step, &u1.d_dt_momentum),
        ),
    );
    let u3 = EquationsOfMotion::new(
        &config,
        &vec_add(
            &state.displacement,
            &vec_scalar_mul(0.5 * time_step, &u2.d_dt_displacement),
        ),
        &vec_add(
            &state.momentum,
            &vec_scalar_mul(0.5 * time_step, &u2.d_dt_momentum),
        ),
    );
    let u4 = EquationsOfMotion::new(
        &config,
        &vec_add(
            &state.displacement,
            &vec_scalar_mul(time_step, &u3.d_dt_displacement),
        ),
        &vec_add(
            &state.momentum,
            &vec_scalar_mul(time_step, &u3.d_dt_momentum),
        ),
    );
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);
    return State {
        displacement: vec_add(&state.displacement, &rk4.d_dt_displacement),
        momentum: vec_add(&state.momentum, &rk4.d_dt_momentum),
        time: state.time + time_step,
    };
}
