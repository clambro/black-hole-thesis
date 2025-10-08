use crate::domain::config::Config;
use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::integration::integrate;
use std::f64::consts::PI;

pub fn build_initial_state(config: &Config) -> State {
    let ingoing = get_initial_wave(config);
    let outgoing = ingoing.clone();

    let constraints = compute_constraints(&ingoing, &outgoing, config);
    let alternate_mass = constraints.mass.clone();

    return State {
        time: 0.0,
        ingoing,
        outgoing,
        constraints,
        alternate_mass,
    };
}

pub fn build_subsequent_state(
    config: &Config,
    time: f64,
    ingoing: FieldVector,
    outgoing: FieldVector,
    alternate_mass: FieldVector,
) -> State {
    let constraints = compute_constraints(&ingoing, &outgoing, config);

    return State {
        time,
        ingoing,
        outgoing,
        constraints,
        alternate_mass,
    };
}

pub fn compute_constraints(
    ingoing: &FieldVector,
    outgoing: &FieldVector,
    config: &Config,
) -> Constraints {
    let lapse = compute_lapse(ingoing, outgoing, config);
    let energy_density = compute_energy_density(ingoing, outgoing, config);
    let radial_factor = compute_radial_factor(&lapse, config);
    let mass = 0.5 * &config.grid.points * (1.0 - &radial_factor);
    let char_speed = &radial_factor / &lapse;

    return Constraints {
        energy_density,
        mass,
        radial_factor,
        lapse,
        char_speed,
    };
}

fn get_initial_wave(config: &Config) -> FieldVector {
    let exponent = -64.0 * (PI / 2.0 * &config.grid.points).tan().powi(2);
    return config.initial_amplitude * exponent.exp();
}

fn compute_energy_density(
    ingoing: &FieldVector,
    outgoing: &FieldVector,
    config: &Config,
) -> FieldVector {
    0.25 * &config.grid.points.powi(2) * (&ingoing.powi(2) + &outgoing.powi(2))
}

fn compute_radial_factor(lapse: &FieldVector, config: &Config) -> FieldVector {
    let indefinite_integral = integrate(&lapse.powi(-1), config.grid.delta);
    let mut radial_factor =
        lapse / &config.grid.points * (&indefinite_integral - indefinite_integral[0]);
    radial_factor[0] = 1.0; // The mass is O(r^3) at the left boundary, so A(0) = 1.
    return radial_factor;
}

fn compute_lapse(ingoing: &FieldVector, outgoing: &FieldVector, config: &Config) -> FieldVector {
    let integrand = -0.5 * &config.grid.points * (&ingoing.powi(2) + &outgoing.powi(2));
    let log_lapse = integrate(&integrand, config.grid.delta);
    let lapse = log_lapse.exp();
    // The lapse is free up to a constant multiplier caused by the choice of reference frame,
    // so we apply the rightmost BC N(1) = 1 to make our time coordinate the proper time at r = 1.
    return 1.0 / lapse[lapse.len() - 1] * lapse;
}
