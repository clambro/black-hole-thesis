use crate::domain::config::Config;
use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::integration::integrate_cumulative;

pub fn compute_constraints(state: &State, config: &Config) -> Constraints {
    let mass = compute_mass(state, config);
    let radial_factor = 1.0 - 2.0 * &mass / &config.grid.points;
    let lapse = compute_lapse(state, config);
    let char_speed = &radial_factor / &lapse;
    return Constraints {
        mass,
        radial_factor,
        lapse,
        char_speed,
    };
}

fn compute_mass(state: &State, config: &Config) -> FieldVector {
    let integrand = config.grid.points.powi(2) * (&state.ingoing.powi(2) + &state.outgoing.powi(2));
    // BC of m(0) = 0 is set automatically because the integrand is 0 at the left boundary and we
    // integrate from left to right.
    return integrate_cumulative(&integrand, config.grid.delta);
}

fn compute_lapse(state: &State, config: &Config) -> FieldVector {
    let integrand = -2.0 * &config.grid.points * (&state.ingoing.powi(2) + &state.outgoing.powi(2));
    let log_lapse = integrate_cumulative(&integrand, config.grid.delta);
    let lapse = log_lapse.exp();
    // The lapse is free up to a constant multiplier caused by the choice of reference frame,
    // so we apply the rightmost BC N(1) = 1 to make it the proper time at r = 1.
    return 1.0 / lapse[lapse.len() - 1] * lapse;
}
