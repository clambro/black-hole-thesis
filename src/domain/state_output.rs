use crate::{
    domain::{config::Config, field_vector::FieldVector, grid::Grid, state::State},
    use_cases::{equations::EquationsOfMotion, integration::integrate_scalar},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct StateOutput {
    pub time: f64,
    pub displacement: Vec<f64>,
    pub momentum: Vec<f64>,
    pub energy_density: Vec<f64>,
    pub total_energy: f64,
}

impl StateOutput {
    pub fn from_state(state: &State, config: &Config) -> Self {
        let energy_density = EquationsOfMotion::calculate_energy_density(&state, &config);
        Self {
            time: state.time,
            displacement: Self::reduce_spatial_resolution(
                &state.displacement,
                config.output_dx_level,
            ),
            momentum: Self::reduce_spatial_resolution(&state.momentum, config.output_dx_level),
            energy_density: Self::reduce_spatial_resolution(
                &energy_density,
                config.output_dx_level,
            ),
            total_energy: integrate_scalar(&energy_density, config.grid.delta),
        }
    }

    fn reduce_spatial_resolution(field: &FieldVector, target_discretization: u32) -> Vec<f64> {
        let current_length = field.len();
        let target_length = Grid::get_length_at_discretization(target_discretization);

        if target_length > current_length {
            panic!("Target resolution is greater than the current resolution.");
        }

        let step = (current_length - 1) / (target_length - 1); // Preserves grid points.

        return field
            .iter()
            .step_by(step)
            .copied()
            .take(target_length)
            .collect();
    }
}
