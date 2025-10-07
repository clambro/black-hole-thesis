use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;

#[derive(Debug)]
pub struct State {
    pub time: f64,
    pub radial_gradient: FieldVector,
    pub conj_momentum: FieldVector,
    pub constraints: Constraints,
}

impl State {
    pub fn get_black_hole_mass(&self) -> Option<f64> {
        return self.constraints.get_black_hole_mass();
    }
}
