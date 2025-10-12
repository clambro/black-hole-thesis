use crate::domain::constants::INITIAL_WAVE_STEEPNESS;
use crate::domain::field_vector::FieldVector;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::state::State;
use crate::use_cases::constraint_computer::compute_constraints;
use std::f64::consts::PI;

/// Build the initial state with a Gaussian wave profile.
pub fn build_initial_state(config: &SimulationConfig) -> State {
    let n = config.grid.len();

    let field = FieldVector::zeros(n);
    let conj_momentum = initial_wave_profile(config);

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

/// Build a subsequent state from evolved field data.
pub fn build_subsequent_state(
    config: &SimulationConfig,
    time: f64,
    field: FieldVector,
    conj_momentum: FieldVector,
    alternate_mass: FieldVector,
) -> State {
    // Panic if any field values are NaN
    if field.iter().any(|&x| x.is_nan()) {
        panic!("Field contains NaN values");
    }

    let constraints = compute_constraints(&field, &conj_momentum, config);

    State {
        time,
        field,
        conj_momentum,
        constraints,
        alternate_mass,
    }
}

/// Create the initial Gaussian wave profile.
fn initial_wave_profile(config: &SimulationConfig) -> FieldVector {
    let exponent = -INITIAL_WAVE_STEEPNESS * (PI / 2.0 * &config.grid.points).tan().powi(2);
    config.initial_amplitude * exponent.exp()
}
