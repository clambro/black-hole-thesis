use crate::domain::{config::Config, field_vector::FieldVector, grid::Grid, state::State};
use serde::Serialize;

#[derive(Serialize)]
pub struct StateOutput {
    pub time: f64,
    pub displacement: Vec<f64>,
    pub momentum: Vec<f64>,
    pub energy_density: Vec<f64>,
}

impl StateOutput {
    pub fn from_state(state: &State, config: &Config) -> Self {
        Self {
            time: state.time,
            displacement: Self::reduce_spatial_resolution(
                &state.displacement,
                config.output_dx_level,
            ),
            momentum: Self::reduce_spatial_resolution(&state.momentum, config.output_dx_level),
            energy_density: Self::reduce_spatial_resolution(
                &Self::calculate_energy_density(&state, &config),
                config.output_dx_level,
            ),
        }
    }

    fn reduce_spatial_resolution(field: &FieldVector, target_discretization: u32) -> Vec<f64> {
        let current_length = field.len();
        let target_length = Grid::get_length_at_discretization(target_discretization);

        if target_length > current_length {
            panic!("Target resolution is greater than the current resolution.");
        }

        let step = current_length / target_length;
        field
            .iter()
            .step_by(step)
            .copied()
            .take(target_length)
            .collect()
    }

    fn calculate_energy_density(state: &State, config: &Config) -> FieldVector {
        let energy_density = 0.5
            * (state.momentum.clone().powi(2)
                + state.displacement.clone().powi(2) * config.wave_speed.powi(2));
        return energy_density;
    }
}
