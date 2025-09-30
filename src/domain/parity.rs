use crate::domain::field_vector::FieldVector;

pub enum Parity {
    Even,
    Odd,
    Swap(FieldVector),
}
