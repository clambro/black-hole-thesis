use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;

#[derive(Debug)]
pub struct State {
    pub time: f64,
    pub ingoing: FieldVector,
    pub outgoing: FieldVector,
    pub alternate_mass: FieldVector,
    pub constraints: Constraints,
}

impl State {
    pub fn get_black_hole_mass(&self) -> Option<f64> {
        return self.constraints.get_black_hole_mass();
    }

    pub fn get_total_energy(&self) -> f64 {
        self.constraints.mass[self.constraints.mass.len() - 1]
    }

    pub fn get_radial_gradient(&self) -> FieldVector {
        return 0.5 * (&self.outgoing - &self.ingoing);
    }

    pub fn get_conj_momentum(&self) -> FieldVector {
        return 0.5 * (&self.outgoing + &self.ingoing);
    }
}
