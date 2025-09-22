use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;

#[derive(Debug)]
pub struct State {
    pub time: f64,
    pub displacement: FieldVector,
    pub momentum: FieldVector,
}

impl State {
    pub fn get_initial_displacement(grid: &Grid, amplitude: f64) -> FieldVector {
        let boundary_factor = &grid.points.powi(2) * (1.0 - &grid.points).powi(2);
        let gaussian = (-((&grid.points - 0.5) * 10.0).powi(2)).exp();
        let displacement = amplitude * gaussian * boundary_factor;
        return displacement;
    }

    pub fn get_initial_momentum(grid: &Grid) -> FieldVector {
        return FieldVector::zeros(grid.get_length());
    }
}
