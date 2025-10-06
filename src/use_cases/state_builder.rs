use crate::domain::config::Config;
use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::integration::integrate;
use std::f64::consts::PI;

pub fn build_initial_state(config: &Config) -> State {
    let radial_gradient = FieldVector::zeros(config.grid.points.len());
    let conj_momentum = get_initial_conj_momentum(config);
    let constraints = compute_constraints(&radial_gradient, &conj_momentum, config);
    return State {
        time: 0.0,
        radial_gradient,
        conj_momentum,
        constraints,
    };
}

pub fn build_subsequent_state(
    config: &Config,
    time: f64,
    radial_gradient: FieldVector,
    conj_momentum: FieldVector,
) -> State {
    let constraints = compute_constraints(&radial_gradient, &conj_momentum, config);
    return State {
        time,
        radial_gradient,
        conj_momentum,
        constraints,
    };
}

pub fn compute_constraints(
    radial_gradient: &FieldVector,
    conj_momentum: &FieldVector,
    config: &Config,
) -> Constraints {
    let energy_density = compute_energy_density(radial_gradient, conj_momentum, config);
    let mass = compute_mass(&energy_density, config);
    let radial_factor = compute_radial_factor(&mass, config);
    let lapse = compute_lapse(radial_gradient, conj_momentum, config);
    let char_speed = &radial_factor / &lapse;
    return Constraints {
        energy_density,
        mass,
        radial_factor,
        lapse,
        char_speed,
    };
}

fn get_initial_conj_momentum(config: &Config) -> FieldVector {
    let exponent = -64.0 * (PI / 2.0 * &config.grid.points).tan().powi(2);
    return config.initial_amplitude * exponent.exp();
}

fn compute_energy_density(
    radial_gradient: &FieldVector,
    conj_momentum: &FieldVector,
    config: &Config,
) -> FieldVector {
    return 0.5 * config.grid.points.powi(2) * (&radial_gradient.powi(2) + &conj_momentum.powi(2));
}

fn compute_mass(energy_density: &FieldVector, config: &Config) -> FieldVector {
    // BC of m(0) = 0 is set automatically because the integrand is 0 at the left boundary and we
    // integrate from left to right.
    return integrate(&energy_density, config.grid.delta);
}

fn compute_radial_factor(mass: &FieldVector, config: &Config) -> FieldVector {
    let mut radial_factor = 1.0 - 2.0 * mass / &config.grid.points;
    radial_factor[0] = 1.0; // The mass is O(r^3) at the left boundary, so A(0) = 1.
    return radial_factor;
}

fn compute_lapse(
    radial_gradient: &FieldVector,
    conj_momentum: &FieldVector,
    config: &Config,
) -> FieldVector {
    let integrand =
        -1.0 * &config.grid.points * (&radial_gradient.powi(2) + &conj_momentum.powi(2));
    let log_lapse = integrate(&integrand, config.grid.delta);
    let lapse = log_lapse.exp();
    // The lapse is free up to a constant multiplier caused by the choice of reference frame,
    // so we apply the rightmost BC N(1) = 1 to make our time coordinate the proper time at r = 1.
    return 1.0 / lapse[lapse.len() - 1] * lapse;
}
