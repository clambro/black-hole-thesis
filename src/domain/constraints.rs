use crate::domain::{constants::BH_RADIAL_FACTOR_THRESHOLD, field_vector::FieldVector};

#[derive(Debug, Clone)]
pub struct Constraints {
    pub energy_density: FieldVector, // E
    pub mass: FieldVector,           // m
    pub radial_factor: FieldVector,  // A
    pub lapse: FieldVector,          // N
    pub char_speed: FieldVector,     // X
}

impl Constraints {
    pub fn black_hole_mass(&self) -> Option<f64> {
        let bh_radius_index = self
            .radial_factor
            .iter()
            .position(|x| x <= &BH_RADIAL_FACTOR_THRESHOLD);

        let bh_radius_index = bh_radius_index?;

        Some(self.mass[bh_radius_index])
    }
}
