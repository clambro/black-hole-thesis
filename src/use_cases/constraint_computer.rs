use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;
use crate::domain::simulation_config::SimulationConfig;
use crate::use_cases::diff::diff;
use crate::use_cases::integration::integrate;

/// Compute all constraint variables from field data.
pub fn compute_constraints(
    field: &FieldVector,
    conj_momentum: &FieldVector,
    config: &SimulationConfig,
) -> Constraints {
    let radial_gradient = compute_radial_gradient(field, config);
    let lapse = compute_lapse(&radial_gradient, conj_momentum, config);
    let energy_density = compute_energy_density(&radial_gradient, conj_momentum, config);
    // Note that doing this the other way around by computing mass from energy density
    // and then computing A destroys conservation of energy. I'm not sure why.
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

/// Compute the radial gradient of the field.
pub fn compute_radial_gradient(field: &FieldVector, config: &SimulationConfig) -> FieldVector {
    diff(&config.grid, field)
}

/// Compute the energy density field.
fn compute_energy_density(
    radial_gradient: &FieldVector,
    conj_momentum: &FieldVector,
    config: &SimulationConfig,
) -> FieldVector {
    0.5 * &config.grid.points.powi(2) * (&radial_gradient.powi(2) + &conj_momentum.powi(2))
}

/// Compute the radial metric factor A.
fn compute_radial_factor(lapse: &FieldVector, config: &SimulationConfig) -> FieldVector {
    let indefinite_integral = integrate(&lapse.powi(-1), config.grid.delta);
    let mut radial_factor =
        lapse / &config.grid.points * (&indefinite_integral - indefinite_integral[0]);
    radial_factor[0] = 1.0; // The mass is O(r^3) at the left boundary, so A(0) = 1.
    radial_factor
}

/// Compute the lapse function N.
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
