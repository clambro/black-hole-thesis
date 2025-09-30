use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;

#[derive(Debug)]
pub struct State {
    pub time: f64,
    pub ingoing: FieldVector,  // W_+ (speed is negative)
    pub outgoing: FieldVector, // W_- (speed is positive)
    pub constraints: Constraints,
}

impl State {
    pub fn get_radial_gradient(&self) -> FieldVector {
        return 0.5 * (&self.ingoing - &self.outgoing);
    }

    pub fn get_conjugate_momentum(&self) -> FieldVector {
        return 0.5 * (&self.ingoing + &self.outgoing);
    }
}
