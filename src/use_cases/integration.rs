use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::equations::EquationsOfMotion;
use crate::use_cases::state_builder::{build_subsequent_state, compute_constraints};

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let initial_ingoing = state.ingoing.clone();
    let initial_outgoing = state.outgoing.clone();

    let u1 = EquationsOfMotion::new(
        &config,
        initial_ingoing.clone(),
        initial_outgoing.clone(),
        &state.constraints,
    );
    let mut u1_ingoing = &initial_ingoing + 0.5 * time_step * &u1.d_dt_ingoing;
    let mut u1_outgoing = &initial_outgoing + 0.5 * time_step * &u1.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut u1_ingoing, &mut u1_outgoing);
    let u1_constraints = compute_constraints(&u1_ingoing, &u1_outgoing, config);

    let u2 = EquationsOfMotion::new(&config, u1_ingoing, u1_outgoing, &u1_constraints);
    let mut u2_ingoing = &initial_ingoing + 0.5 * time_step * &u2.d_dt_ingoing;
    let mut u2_outgoing = &initial_outgoing + 0.5 * time_step * &u2.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut u2_ingoing, &mut u2_outgoing);
    let u2_constraints = compute_constraints(&u2_ingoing, &u2_outgoing, config);

    let u3 = EquationsOfMotion::new(&config, u2_ingoing, u2_outgoing, &u2_constraints);
    let mut u3_ingoing = &initial_ingoing + time_step * &u3.d_dt_ingoing;
    let mut u3_outgoing = &initial_outgoing + time_step * &u3.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut u3_ingoing, &mut u3_outgoing);
    let u3_constraints = compute_constraints(&u3_ingoing, &u3_outgoing, config);

    let u4 = EquationsOfMotion::new(&config, u3_ingoing, u3_outgoing, &u3_constraints);
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);

    let mut ingoing = &initial_ingoing + &rk4.d_dt_ingoing;
    let mut outgoing = &initial_outgoing + &rk4.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut ingoing, &mut outgoing);

    let time = state.time + time_step;
    return build_subsequent_state(config, time, ingoing, outgoing);
}

/// Integrate a vector cumulatively to a vector using Simpson's rule (4th order accurate).
pub fn integrate(vector: &FieldVector, grid_size: f64) -> FieldVector {
    let n = vector.len();
    let mut result = FieldVector::zeros(n);

    // The left BC is set to zero by default, but integrals are free up to an additive constant.
    // You can thus add whatever constant you want to the result to shift it up or down.
    result[1] = grid_size / 12.0 * (5.0 * vector[0] + 8.0 * vector[1] - vector[2]);
    result[2] = grid_size / 3.0 * (vector[0] + 4.0 * vector[1] + vector[2]);

    for i in 3..n {
        result[i] =
            result[i - 2] + grid_size / 3.0 * (vector[i - 2] + 4.0 * vector[i - 1] + vector[i]);
    }
    return result;
}
