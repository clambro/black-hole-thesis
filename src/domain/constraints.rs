use crate::domain::field_vector::FieldVector;

pub struct Constraints {
    pub mass: FieldVector,          // m
    pub radial_factor: FieldVector, // A
    pub lapse: FieldVector,         // N
    pub char_speed: FieldVector,    // X
}
