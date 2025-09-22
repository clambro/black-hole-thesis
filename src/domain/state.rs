use crate::domain::boundary_conditions::{BoundaryCondition, BoundaryConditions};
use crate::domain::grid::Grid;

#[derive(Debug)]
pub struct State {
    pub grid: Grid,
    pub boundary_conditions: BoundaryConditions,
    pub wave_speed: f64,
    pub wave_position: Vec<f64>,
    pub wave_velocity: Vec<f64>,
    pub courant: f64,
    pub total_time: f64,
}

impl State {
    pub fn from_args(
        grid: Grid,
        wave_speed: f64,
        amplitude: f64,
        left_bc: BoundaryCondition,
        right_bc: BoundaryCondition,
        courant: f64,
        total_time: f64,
    ) -> Self {
        // Assume a Gaussian wave packet.
        let grid_size = grid.points.len();
        let wave_position: Vec<f64> = grid
            .points
            .iter()
            .map(|x| {
                // Polynomial: x^2 * (1-x)^2 gives f(0)=f(1)=0 and f'(0)=f'(1)=0
                // Satisifies both Neumann and Dirichlet boundary conditions.
                let boundary_factor = x.powi(2) * (1.0 - x).powi(2);

                // Gaussian envelope centered at 0.5 gives the basic wave packet.
                let gaussian = (-((x - 0.5) * 10.0).powi(2)).exp();

                amplitude * gaussian * boundary_factor
            })
            .collect();
        Self {
            grid,
            boundary_conditions: BoundaryConditions {
                left: left_bc,
                right: right_bc,
            },
            wave_speed,
            wave_position,
            wave_velocity: vec![0.0; grid_size],
            courant,
            total_time,
        }
    }
}
