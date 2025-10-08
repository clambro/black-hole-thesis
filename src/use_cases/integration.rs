use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::equations::EquationsOfMotion;
use crate::use_cases::state_builder::{build_subsequent_state, compute_constraints};

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(
        &config,
        state.radial_gradient.clone(),
        state.conj_momentum.clone(),
        &state.constraints,
    );
    let mut u1_radial_gradient = &state.radial_gradient + 0.5 * time_step * &u1.dt_radial_gradient;
    let mut u1_conj_momentum = &state.conj_momentum + 0.5 * time_step * &u1.dt_conj_momentum;
    EquationsOfMotion::apply_bcs(&mut u1_radial_gradient, &mut u1_conj_momentum);
    let u1_constraints = compute_constraints(&u1_radial_gradient, &u1_conj_momentum, config);

    let u2 = EquationsOfMotion::new(
        &config,
        u1_radial_gradient.clone(),
        u1_conj_momentum.clone(),
        &u1_constraints,
    );
    let mut u2_radial_gradient = &state.radial_gradient + 0.5 * time_step * &u2.dt_radial_gradient;
    let mut u2_conj_momentum = &state.conj_momentum + 0.5 * time_step * &u2.dt_conj_momentum;
    EquationsOfMotion::apply_bcs(&mut u2_radial_gradient, &mut u2_conj_momentum);
    let u2_constraints = compute_constraints(&u2_radial_gradient, &u2_conj_momentum, config);

    let u3 = EquationsOfMotion::new(
        &config,
        u2_radial_gradient.clone(),
        u2_conj_momentum.clone(),
        &u2_constraints,
    );
    let mut u3_radial_gradient = &state.radial_gradient + time_step * &u3.dt_radial_gradient;
    let mut u3_conj_momentum = &state.conj_momentum + time_step * &u3.dt_conj_momentum;
    EquationsOfMotion::apply_bcs(&mut u3_radial_gradient, &mut u3_conj_momentum);
    let u3_constraints = compute_constraints(&u3_radial_gradient, &u3_conj_momentum, config);

    let u4 = EquationsOfMotion::new(
        &config,
        u3_radial_gradient.clone(),
        u3_conj_momentum.clone(),
        &u3_constraints,
    );
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);

    let mut radial_gradient = &state.radial_gradient + &rk4.dt_radial_gradient;
    let mut conj_momentum = &state.conj_momentum + &rk4.dt_conj_momentum;
    let alternate_mass = &state.alternate_mass + &rk4.dt_alternate_mass;
    EquationsOfMotion::apply_bcs(&mut radial_gradient, &mut conj_momentum);

    let time = state.time + time_step;

    return build_subsequent_state(config, time, radial_gradient, conj_momentum, alternate_mass);
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
