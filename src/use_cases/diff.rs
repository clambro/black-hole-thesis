use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;

/// SBP 4th order finite difference operator (provably stable, conserves energy).
pub fn diff(grid: &Grid, vector: &FieldVector) -> FieldVector {
    let mut diff = FieldVector::zeros(vector.len());
    let n = vector.len();
    let h = grid.delta;

    // Left boundary.
    diff[0] = (-24.0 / 17.0 * vector[0] + 59.0 / 34.0 * vector[1]
        - 4.0 / 17.0 * vector[2]
        - 3.0 / 34.0 * vector[3])
        / h;

    diff[1] = (-1.0 / 2.0 * vector[0] + 1.0 / 2.0 * vector[2]) / h;

    diff[2] = (4.0 / 43.0 * vector[0] - 59.0 / 86.0 * vector[1] + 59.0 / 86.0 * vector[3]
        - 4.0 / 43.0 * vector[4])
        / h;

    diff[3] = (3.0 / 98.0 * vector[0] - 59.0 / 98.0 * vector[2] + 32.0 / 49.0 * vector[4]
        - 4.0 / 49.0 * vector[5])
        / h;

    // Interior points.
    for i in 4..n - 4 {
        diff[i] = (vector[i - 2] - 8.0 * vector[i - 1] + 8.0 * vector[i + 1] - vector[i + 2])
            / (12.0 * h);
    }

    // Right boundary.
    diff[n - 1] = (24.0 / 17.0 * vector[n - 1] - 59.0 / 34.0 * vector[n - 2]
        + 4.0 / 17.0 * vector[n - 3]
        + 3.0 / 34.0 * vector[n - 4])
        / h;

    diff[n - 2] = (1.0 / 2.0 * vector[n - 1] - 1.0 / 2.0 * vector[n - 3]) / h;

    diff[n - 3] = (-4.0 / 43.0 * vector[n - 1] + 59.0 / 86.0 * vector[n - 2]
        - 59.0 / 86.0 * vector[n - 4]
        + 4.0 / 43.0 * vector[n - 5])
        / h;

    diff[n - 4] = (-3.0 / 98.0 * vector[n - 1] + 59.0 / 98.0 * vector[n - 3]
        - 32.0 / 49.0 * vector[n - 5]
        + 4.0 / 49.0 * vector[n - 6])
        / h;

    return diff;
}
