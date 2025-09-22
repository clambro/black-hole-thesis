#[derive(Debug, Clone)]
pub struct Grid {
    pub points: Vec<f64>,
    pub delta: f64,
}

impl Grid {
    pub fn from_level_of_discretization(level: u32) -> Self {
        let num_points: u32 = 2_u32.pow(level) + 1;
        let delta: f64 = 2_f64.powi(-(level as i32)); // Forcing 0 to 1 inclusive.
        let points: Vec<f64> = (0..num_points).map(|i| i as f64 * delta).collect();
        return Self { points, delta };
    }
}
