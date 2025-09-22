use crate::domain::grid::Grid;

#[derive(Debug)]
pub struct State {
    pub time: f64,
    pub displacement: Vec<f64>,
    pub momentum: Vec<f64>,
}

impl State {
    pub fn get_initial_displacement(grid: &Grid,
        amplitude: f64) -> Vec<f64> {
        let displacement: Vec<f64> = grid
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
        return displacement;
    }

    pub fn get_initial_momentum(grid: &Grid) -> Vec<f64> {
        return vec![0.0; grid.points.len()];
    }
}
