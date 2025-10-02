use crate::domain::field_vector::FieldVector;

#[derive(Debug)]
pub struct Constraints {
    pub mass: FieldVector,          // m
    pub radial_factor: FieldVector, // A
    pub lapse: FieldVector,         // N
    pub char_speed: FieldVector,    // X
}
