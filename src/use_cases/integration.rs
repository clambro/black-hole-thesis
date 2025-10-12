use crate::domain::field_vector::FieldVector;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::state::State;
use crate::use_cases::constraint_computer::compute_constraints;
use crate::use_cases::equations::EquationsOfMotion;
use crate::use_cases::state_builder::build_subsequent_state;

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &SimulationConfig, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(
        config,
        &state.field,
        &state.conj_momentum,
        &state.constraints,
    );
    let mut u1_field = &state.field + 0.5 * time_step * &u1.dt_field;
    let u1_conj_momentum = &state.conj_momentum + 0.5 * time_step * &u1.dt_conj_momentum;
    EquationsOfMotion::apply_bcs(&mut u1_field);
    let u1_constraints = compute_constraints(&u1_field, &u1_conj_momentum, config);

    let u2 = EquationsOfMotion::new(config, &u1_field, &u1_conj_momentum, &u1_constraints);
    let mut u2_field = &state.field + 0.5 * time_step * &u2.dt_field;
    let u2_conj_momentum = &state.conj_momentum + 0.5 * time_step * &u2.dt_conj_momentum;
    EquationsOfMotion::apply_bcs(&mut u2_field);
    let u2_constraints = compute_constraints(&u2_field, &u2_conj_momentum, config);

    let u3 = EquationsOfMotion::new(config, &u2_field, &u2_conj_momentum, &u2_constraints);
    let mut u3_field = &state.field + time_step * &u3.dt_field;
    let u3_conj_momentum = &state.conj_momentum + time_step * &u3.dt_conj_momentum;
    EquationsOfMotion::apply_bcs(&mut u3_field);
    let u3_constraints = compute_constraints(&u3_field, &u3_conj_momentum, config);

    let u4 = EquationsOfMotion::new(config, &u3_field, &u3_conj_momentum, &u3_constraints);
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);

    let mut field = &state.field + &rk4.dt_field;
    let conj_momentum = &state.conj_momentum + &rk4.dt_conj_momentum;
    let alternate_mass = &state.alternate_mass + &rk4.dt_alternate_mass;
    EquationsOfMotion::apply_bcs(&mut field);

    let time = state.time + time_step;

    build_subsequent_state(config, time, field, conj_momentum, alternate_mass)
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
        if i % 2 == 0 {
            result[i] =
                result[i - 2] + grid_size / 3.0 * (vector[i - 2] + 4.0 * vector[i - 1] + vector[i]);
        } else {
            result[i] = result[i - 3]
                + 3.0 * grid_size / 8.0
                    * (vector[i - 3] + 3.0 * vector[i - 2] + 3.0 * vector[i - 1] + vector[i]);
        }
    }
    result
}
