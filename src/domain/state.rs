use crate::domain::config::Config;
use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;
use crate::domain::mass_history::MassHistory;

#[derive(Debug)]
pub struct State {
    pub time: f64,
    pub radial_gradient: FieldVector,
    pub conj_momentum: FieldVector,
    pub constraints: Constraints,
    pub mass_history: MassHistory,
}

impl State {
    pub fn get_black_hole_mass(&self) -> Option<f64> {
        return self.constraints.get_black_hole_mass();
    }

    pub fn calculate_momentum_residual(&self, config: &Config) -> FieldVector {
        let mass_time_derivative = self.mass_history.calculate_mass_time_derivative();
        let expected_momentum_flux = config.grid.points.powi(2)
            * &self.constraints.radial_factor.powi(2)
            / &self.constraints.lapse
            * &self.radial_gradient
            * &self.conj_momentum;

        return expected_momentum_flux - mass_time_derivative;
    }
}
