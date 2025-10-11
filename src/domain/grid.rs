use crate::domain::field_vector::FieldVector;

/// Spatial grid for the simulation domain.
#[derive(Debug, Clone)]
pub struct Grid {
    /// Grid point coordinates.
    pub points: FieldVector,
    /// Grid spacing.
    pub delta: f64,
    /// Discretization level.
    pub level: u32,
}

impl Grid {
    /// Create a grid from the discretization level.
    pub fn from_level_of_discretization(level: u32) -> Self {
        let num_points: usize = Grid::length_at_discretization(level);
        let delta: f64 = 2_f64.powi(-(level as i32)); // Forcing 0 to 1 inclusive.
        let points: FieldVector =
            FieldVector::new((0..num_points).map(|i| i as f64 * delta).collect());
        Self {
            points,
            delta,
            level,
        }
    }

    /// Get the number of grid points.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Calculate the number of points for a given discretization level.
    pub fn length_at_discretization(discretization: u32) -> usize {
        2_usize.pow(discretization) + 1
    }
}
