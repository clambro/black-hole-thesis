use serde::Serialize;

/// Serialized state output for file storage.
#[derive(Serialize)]
pub struct StateOutput {
    /// Current simulation time.
    pub time: f64,
    /// Scalar field values phi.
    pub field: Vec<f64>,
    /// Radial gradient of the field Phi.
    pub radial_gradient: Vec<f64>,
    /// Conjugate momentum values Pi.
    pub conj_momentum: Vec<f64>,
    /// Mass function values m.
    pub mass: Vec<f64>,
    /// Radial metric factor values A.
    pub radial_factor: Vec<f64>,
    /// Lapse function values N.
    pub lapse: Vec<f64>,
    /// Characteristic speed values X = A/N.
    pub char_speed: Vec<f64>,
    /// Energy density values rho.
    pub energy_density: Vec<f64>,
    /// Total energy in the system.
    pub total_energy: f64,
    /// Alternate mass function for diagnostics.
    pub alternate_mass: Vec<f64>,
}
