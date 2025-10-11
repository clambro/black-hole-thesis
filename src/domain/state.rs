use crate::domain::constraints::Constraints;
use crate::domain::field_vector::FieldVector;

/// Complete simulation state at a point in time.
#[derive(Debug, Clone)]
pub struct State {
    /// Current simulation time.
    pub time: f64,
    /// Scalar field values phi.
    pub field: FieldVector,
    /// Conjugate momentum field values Pi.
    pub conj_momentum: FieldVector,
    /// Alternate mass function for diagnostics.
    pub alternate_mass: FieldVector,
    /// Computed constraint variables.
    pub constraints: Constraints,
}

impl State {
    /// Get the black hole mass if one has formed.
    pub fn black_hole_mass(&self) -> Option<f64> {
        self.constraints.black_hole_mass()
    }

    /// Get the total energy in the system.
    pub fn total_energy(&self) -> f64 {
        self.constraints.mass[self.constraints.mass.len() - 1]
    }
}
