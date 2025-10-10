use crate::domain::constants::INITIAL_WAVE_EXPONENT;
use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::state::State;
use crate::use_cases::diff::diff;
use crate::use_cases::integration::integrate;
use std::f64::consts::PI;

pub fn build_initial_state(config: &SimulationConfig) -> State {
    let n = config.grid.points.len();

    let field = FieldVector::zeros(n);
    let conj_momentum = get_initial_wave(config);

    let constraints = compute_constraints(&field, &conj_momentum, config);
    let alternate_mass = constraints.mass.clone();

    State {
        time: 0.0,
        field,
        conj_momentum,
        constraints,
        alternate_mass,
    }
}

pub fn build_subsequent_state(
    config: &SimulationConfig,
    time: f64,
    field: FieldVector,
    conj_momentum: FieldVector,
    alternate_mass: FieldVector,
) -> State {
    let constraints = compute_constraints(&field, &conj_momentum, config);

    State {
        time,
        field,
        conj_momentum,
        constraints,
        alternate_mass,
    }
}

pub fn compute_constraints(
    field: &FieldVector,
    conj_momentum: &FieldVector,
    config: &SimulationConfig,
) -> Constraints {
    let radial_gradient = compute_radial_gradient(field, config);
    let lapse = compute_lapse(&radial_gradient, conj_momentum, config);
    let energy_density = compute_energy_density(&radial_gradient, conj_momentum, config);
    let radial_factor = compute_radial_factor(&lapse, config);
    let mass = 0.5 * &config.grid.points * (1.0 - &radial_factor);
    let char_speed = &radial_factor / &lapse;

    Constraints {
        energy_density,
        mass,
        radial_factor,
        lapse,
        char_speed,
    }
}

fn compute_radial_gradient(field: &FieldVector, config: &SimulationConfig) -> FieldVector {
    diff(&config.grid, field)
}

fn get_initial_wave(config: &SimulationConfig) -> FieldVector {
    let exponent = -INITIAL_WAVE_EXPONENT * (PI / 2.0 * &config.grid.points).tan().powi(2);
    config.initial_amplitude * exponent.exp()
}

fn compute_energy_density(
    radial_gradient: &FieldVector,
    conj_momentum: &FieldVector,
    config: &SimulationConfig,
) -> FieldVector {
    0.5 * &config.grid.points.powi(2) * (&radial_gradient.powi(2) + &conj_momentum.powi(2))
}

fn compute_radial_factor(lapse: &FieldVector, config: &SimulationConfig) -> FieldVector {
    let indefinite_integral = integrate(&lapse.powi(-1), config.grid.delta);
    let mut radial_factor =
        lapse / &config.grid.points * (&indefinite_integral - indefinite_integral[0]);
    radial_factor[0] = 1.0; // The mass is O(r^3) at the left boundary, so A(0) = 1.
    radial_factor
}

fn compute_lapse(
    radial_gradient: &FieldVector,
    conj_momentum: &FieldVector,
    config: &SimulationConfig,
) -> FieldVector {
    let integrand =
        -1.0 * &config.grid.points * (&radial_gradient.powi(2) + &conj_momentum.powi(2));
    let log_lapse = integrate(&integrand, config.grid.delta);
    let lapse = log_lapse.exp();
    // The lapse is free up to a constant multiplier caused by the choice of reference frame,
    // so we apply the rightmost BC N(1) = 1 to make our time coordinate the proper time at r = 1.
    1.0 / lapse[lapse.len() - 1] * lapse
}
