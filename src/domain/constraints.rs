use crate::domain::{constants::BH_RADIAL_FACTOR_THRESHOLD, field_vector::FieldVector};

/// Computed constraint variables from the field state.
#[derive(Debug, Clone)]
pub struct Constraints {
    /// Energy density field rho.
    pub energy_density: FieldVector,
    /// Mass function m.
    pub mass: FieldVector,
    /// Radial metric factor A.
    pub radial_factor: FieldVector,
    /// Lapse function N.
    pub lapse: FieldVector,
    /// Characteristic speed X = A/N.
    pub char_speed: FieldVector,
}

impl Constraints {
    /// Get the black hole mass if one has formed.
    pub fn black_hole_mass(&self) -> Option<f64> {
        let bh_radius_index = self
            .radial_factor
            .iter()
            .position(|x| x <= &BH_RADIAL_FACTOR_THRESHOLD);

        let bh_radius_index = bh_radius_index?;

        Some(self.mass[bh_radius_index])
    }
}
