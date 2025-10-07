use crate::domain::{config::Config, field_vector::FieldVector, grid::Grid, state::State};
use serde::Serialize;

#[derive(Serialize)]
pub struct StateOutput {
    pub time: f64,
    pub radial_gradient: Vec<f64>,
    pub conj_momentum: Vec<f64>,
    pub mass: Vec<f64>,
    pub compactness: Vec<f64>,
    pub lapse: Vec<f64>,
    pub char_speed: Vec<f64>,
    pub energy_density: Vec<f64>,
    pub total_energy: f64,
}

impl StateOutput {
    pub fn from_state(state: &State, config: &Config) -> Self {
        let level = config.output_dx_level;
        Self {
            time: state.time,
            radial_gradient: Self::reduce_spatial_resolution(&state.radial_gradient, level),
            conj_momentum: Self::reduce_spatial_resolution(&state.conj_momentum, level),
            mass: Self::reduce_spatial_resolution(&state.constraints.mass, level),
            compactness: Self::reduce_spatial_resolution(
                &(1.0 - &state.constraints.radial_factor),
                level,
            ),
            lapse: Self::reduce_spatial_resolution(&state.constraints.lapse, level),
            char_speed: Self::reduce_spatial_resolution(&state.constraints.char_speed, level),
            energy_density: Self::reduce_spatial_resolution(
                &state.constraints.energy_density,
                level,
            ),
            total_energy: state.constraints.mass[state.constraints.mass.len() - 1], // E = m because c = 1.
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
