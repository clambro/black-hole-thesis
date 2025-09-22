use crate::domain::config::Config;
use crate::domain::state::State;
use crate::use_cases::equations::EquationsOfMotion;

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(&config, state.displacement.clone(), state.momentum.clone());
    let u2 = EquationsOfMotion::new(
        &config,
        &state.displacement + 0.5 * time_step * &u1.d_dt_displacement,
        &state.momentum + 0.5 * time_step * &u1.d_dt_momentum,
    );
    let u3 = EquationsOfMotion::new(
        &config,
        &state.displacement + 0.5 * time_step * &u2.d_dt_displacement,
        &state.momentum + 0.5 * time_step * &u2.d_dt_momentum,
    );
    let u4 = EquationsOfMotion::new(
        &config,
        &state.displacement + time_step * &u3.d_dt_displacement,
        &state.momentum + time_step * &u3.d_dt_momentum,
    );
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);
    return State {
        displacement: &state.displacement + &rk4.d_dt_displacement,
        momentum: &state.momentum + &rk4.d_dt_momentum,
        time: state.time + time_step,
    };
}
