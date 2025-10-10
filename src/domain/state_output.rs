use serde::Serialize;

#[derive(Serialize)]
pub struct StateOutput {
    pub time: f64,
    pub field: Vec<f64>,
    pub radial_gradient: Vec<f64>,
    pub conj_momentum: Vec<f64>,
    pub mass: Vec<f64>,
    pub compactness: Vec<f64>,
    pub lapse: Vec<f64>,
    pub char_speed: Vec<f64>,
    pub energy_density: Vec<f64>,
    pub total_energy: f64,
    pub alternate_mass: Vec<f64>,
}
