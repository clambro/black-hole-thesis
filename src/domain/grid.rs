use crate::domain::field_vector::FieldVector;

#[derive(Debug, Clone)]
pub struct Grid {
    pub points: FieldVector,
    pub delta: f64,
    pub level: u32,
}

impl Grid {
    pub fn from_level_of_discretization(level: u32) -> Self {
        let num_points: usize = Grid::get_length_at_discretization(level);
        let delta: f64 = 2_f64.powi(-(level as i32)); // Forcing 0 to 1 inclusive.
        let points: FieldVector =
            FieldVector::new((0..num_points).map(|i| i as f64 * delta).collect());
         Self {
            points,
            delta,
            level,
        }
    }

    pub fn get_length(&self) -> usize {
         self.points.len()
    }

    pub fn get_length_at_discretization(discretization: u32) -> usize {
         2_usize.pow(discretization) + 1
    }
}
