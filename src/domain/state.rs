use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;

#[derive(Debug, Clone)]
pub struct State {
    pub time: f64,
    pub field: FieldVector,
    pub conj_momentum: FieldVector,
    pub alternate_mass: FieldVector,
    pub constraints: Constraints,
}

impl State {
    pub fn black_hole_mass(&self) -> Option<f64> {
        self.constraints.black_hole_mass()
    }

    pub fn total_energy(&self) -> f64 {
        self.constraints.mass[self.constraints.mass.len() - 1]
    }
}
