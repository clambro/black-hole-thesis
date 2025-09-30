use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::equations::EquationsOfMotion;
use crate::use_cases::state_builder::{build_subsequent_state, compute_constraints};

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(
        &config,
        state.ingoing.clone(),
        state.outgoing.clone(),
        &state.constraints,
        time_step,
    );
    let mut u1_ingoing = &state.ingoing + 0.5 * time_step * &u1.d_dt_ingoing;
    let mut u1_outgoing = &state.outgoing + 0.5 * time_step * &u1.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut u1_ingoing, &mut u1_outgoing);
    let u1_constraints = compute_constraints(&u1_ingoing, &u1_outgoing, config);

    let u2 = EquationsOfMotion::new(&config, u1_ingoing, u1_outgoing, &u1_constraints, time_step);
    let mut u2_ingoing = &state.ingoing + 0.5 * time_step * &u2.d_dt_ingoing;
    let mut u2_outgoing = &state.outgoing + 0.5 * time_step * &u2.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut u2_ingoing, &mut u2_outgoing);
    let u2_constraints = compute_constraints(&u2_ingoing, &u2_outgoing, config);

    let u3 = EquationsOfMotion::new(&config, u2_ingoing, u2_outgoing, &u2_constraints, time_step);
    let mut u3_ingoing = &state.ingoing + time_step * &u3.d_dt_ingoing;
    let mut u3_outgoing = &state.outgoing + time_step * &u3.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut u3_ingoing, &mut u3_outgoing);
    let u3_constraints = compute_constraints(&u3_ingoing, &u3_outgoing, config);

    let u4 = EquationsOfMotion::new(&config, u3_ingoing, u3_outgoing, &u3_constraints, time_step);
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);

    let mut ingoing = &state.ingoing + &rk4.d_dt_ingoing;
    let mut outgoing = &state.outgoing + &rk4.d_dt_outgoing;
    EquationsOfMotion::apply_bcs(&mut ingoing, &mut outgoing);

    let time = state.time + time_step;
    return build_subsequent_state(config, time, ingoing, outgoing);
}

/// Integrate a vector spatially to a scalar using Simpson's rule (4th order a  ccurate).
pub fn integrate_scalar(vector: &FieldVector, grid_size: f64) -> f64 {
    let n = vector.len();
    let mut sum = vector[0] + vector[n - 1];

    for i in 1..n - 1 {
        if i % 2 == 1 {
            sum += 4.0 * vector[i];
        } else {
            sum += 2.0 * vector[i];
        }
    }

    return sum * grid_size / 3.0;
}

/// Integrate a vector cumulatively to a vector using Simpson's rule (4th order accurate).
pub fn integrate_cumulative(vector: &FieldVector, grid_size: f64) -> FieldVector {
    // TODO: Implement this correctly. This is not 4th order accurate.
    let n = vector.len();
    let mut sum = vector[0];
    let mut result = FieldVector::zeros(n);
    for i in 1..n {
        result[i] = sum;
        sum += vector[i];
    }
    return result * grid_size;
}
