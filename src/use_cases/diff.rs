use crate::domain::grid::Grid;
use rayon::prelude::*;

/// Compute the derivative of a vector using a second order finite difference method.
pub fn diff(grid: &Grid, vector: &Vec<f64>) -> Vec<f64> {
    let mut diff: Vec<f64> = vec![0.0; vector.len()];

    // Boundary terms.
    diff[0] = (-3.0 * vector[0] + 4.0 * vector[1] - vector[2]) / (2.0 * grid.delta);
    diff[vector.len() - 1] = (3.0 * vector[vector.len() - 1] - 4.0 * vector[vector.len() - 2]
        + vector[vector.len() - 3])
        / (2.0 * grid.delta);

    // Interior terms.
    diff[1..vector.len() - 1]
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, diff_val)| {
            let i = idx + 1; // Zero is handled above.
            *diff_val = (vector[i + 1] - vector[i - 1]) / (2.0 * grid.delta);
        });

    return diff;
}
