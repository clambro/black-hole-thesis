use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;
use crate::domain::output_config::OutputConfig;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::state::State;
use crate::domain::state_output::StateOutput;
use crate::use_cases::constraint_computer::compute_radial_gradient;

/// Build a serializable state output from the simulation state.
pub fn build_state_output(
    state: &State,
    output_config: &OutputConfig,
    simulation_config: &SimulationConfig,
) -> StateOutput {
    let level = output_config.dx_level;
    let radial_gradient = compute_radial_gradient(&state.field, simulation_config);

    StateOutput {
        time: state.time,
        field: reduce_spatial_resolution(&state.field, level),
        radial_gradient: reduce_spatial_resolution(&radial_gradient, level),
        conj_momentum: reduce_spatial_resolution(&state.conj_momentum, level),
        mass: reduce_spatial_resolution(&state.constraints.mass, level),
        radial_factor: reduce_spatial_resolution(&state.constraints.radial_factor, level),
        lapse: reduce_spatial_resolution(&state.constraints.lapse, level),
        char_speed: reduce_spatial_resolution(&state.constraints.char_speed, level),
        energy_density: reduce_spatial_resolution(&state.constraints.energy_density, level),
        total_energy: state.total_energy(),
        alternate_mass: reduce_spatial_resolution(&state.alternate_mass, level),
    }
}

/// Reduce spatial resolution for output data.
fn reduce_spatial_resolution(field: &FieldVector, target_discretization: u32) -> Vec<f64> {
    let current_length = field.len();
    let target_length = Grid::length_at_discretization(target_discretization);

    // Initialization validation ensures that target_length is less than current_length.
    let step = (current_length - 1) / (target_length - 1); // Preserves grid points.

    field
        .iter()
        .step_by(step)
        .copied()
        .take(target_length)
        .collect()
}
