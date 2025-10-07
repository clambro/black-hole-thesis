use crate::domain::field_vector::FieldVector;

#[derive(Debug)]
pub struct Constraints {
    pub energy_density: FieldVector, // E
    pub mass: FieldVector,           // m
    pub radial_factor: FieldVector,  // A
    pub lapse: FieldVector,          // N
    pub char_speed: FieldVector,     // X
}

impl Constraints {
    pub fn get_black_hole_mass(&self) -> Option<f64> {
        let bh_radius_index = self.radial_factor.iter().position(|x| x <= &0.05);
        if bh_radius_index.is_none() {
            return None;
        }
        let bh_radius_index = bh_radius_index.unwrap();
        return Some(self.mass[bh_radius_index]);
    }
}
