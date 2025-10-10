use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;

#[derive(Debug)]
pub struct State {
    pub time: f64,
    pub field: FieldVector,
    pub conj_momentum: FieldVector,
    pub radial_gradient: FieldVector,
    pub alternate_mass: FieldVector,
    pub constraints: Constraints,
}

impl State {
    pub fn get_black_hole_mass(&self) -> Option<f64> {
        self.constraints.get_black_hole_mass()
    }

    pub fn get_total_energy(&self) -> f64 {
        self.constraints.mass[self.constraints.mass.len() - 1]
    }
}
