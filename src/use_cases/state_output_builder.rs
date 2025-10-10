use crate::domain::{
    field_vector::FieldVector, output_config::OutputConfig, simulation_config::SimulationConfig,
    state::State, state_output::StateOutput,
};
use crate::use_cases::diff::diff;

pub fn build_state_output(
    state: &State,
    output_config: &OutputConfig,
    simulation_config: &SimulationConfig,
) -> StateOutput {
    let level = output_config.dx_level;
    let radial_gradient = diff(&simulation_config.grid, &state.field);

    StateOutput {
        time: state.time,
        field: reduce_spatial_resolution(&state.field, level),
        radial_gradient: reduce_spatial_resolution(&radial_gradient, level),
        conj_momentum: reduce_spatial_resolution(&state.conj_momentum, level),
        mass: reduce_spatial_resolution(&state.constraints.mass, level),
        compactness: reduce_spatial_resolution(&(1.0 - &state.constraints.radial_factor), level),
        lapse: reduce_spatial_resolution(&state.constraints.lapse, level),
        char_speed: reduce_spatial_resolution(&state.constraints.char_speed, level),
        energy_density: reduce_spatial_resolution(&state.constraints.energy_density, level),
        total_energy: state.total_energy(),
        alternate_mass: reduce_spatial_resolution(&state.alternate_mass, level),
    }
}

fn reduce_spatial_resolution(field: &FieldVector, target_discretization: u32) -> Vec<f64> {
    let current_length = field.len();
    let target_length = crate::domain::grid::Grid::length_at_discretization(target_discretization);

    if target_length > current_length {
        panic!("Target resolution is greater than the current resolution.");
    }

    let step = (current_length - 1) / (target_length - 1); // Preserves grid points.

    field
        .iter()
        .step_by(step)
        .copied()
        .take(target_length)
        .collect()
}
