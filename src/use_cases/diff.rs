use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;

/// Standard 4th order finite difference operator for the first derivative.
/// From https://web.media.mit.edu/~crtaylor/calculator.html
pub fn diff(grid: &Grid, vector: &FieldVector) -> FieldVector {
    let mut diff = FieldVector::zeros(vector.len());
    let n = vector.len();
    let h = grid.delta;

    // Forward difference for the left boundary.
    diff[0] = (-25.0 * vector[0] + 48.0 * vector[1] - 36.0 * vector[2] + 16.0 * vector[3]
        - 3.0 * vector[4])
        / (12.0 * h);

    diff[1] = (-3.0 * vector[0] - 10.0 * vector[1] + 18.0 * vector[2] - 6.0 * vector[3]
        + vector[4])
        / (12.0 * h);

    // Central difference for the interior points.
    (2..n - 2).for_each(|i| {
        diff[i] = (vector[i - 2] - 8.0 * vector[i - 1] + 8.0 * vector[i + 1] - vector[i + 2])
            / (12.0 * h);
    });

    // Backward difference for the right boundary.
    diff[n - 2] = (-vector[n - 5] + 6.0 * vector[n - 4] - 18.0 * vector[n - 3]
        + 10.0 * vector[n - 2]
        + 3.0 * vector[n - 1])
        / (12.0 * h);

    diff[n - 1] = (3.0 * vector[n - 5] - 16.0 * vector[n - 4] + 36.0 * vector[n - 3]
        - 48.0 * vector[n - 2]
        + 25.0 * vector[n - 1])
        / (12.0 * h);

    return diff;
}

/// Apply a 5th order Kreiss-Oliger dissipation operator.
/// This smooths out high frequency noise at the 5th order level without affecting our 4th order accuracy.
/// Stencils also from https://web.media.mit.edu/~crtaylor/calculator.html
pub fn dissipation(vector: &FieldVector, grid: &Grid) -> FieldVector {
    let mut result = FieldVector::zeros(vector.len());
    let n = vector.len();

    // Left boundary.
    (0..3).for_each(|i| {
        result[i] = vector[i] - 6.0 * vector[i + 1] + 15.0 * vector[i + 2] - 20.0 * vector[i + 3]
            + 15.0 * vector[i + 4]
            - 6.0 * vector[i + 5]
            + vector[i + 6];
    });

    // Interior points.
    (3..n - 3).for_each(|i| {
        result[i] = vector[i + 3] - 6.0 * vector[i + 2] + 15.0 * vector[i + 1] - 20.0 * vector[i]
            + 15.0 * vector[i - 1]
            - 6.0 * vector[i - 2]
            + vector[i - 3];
    });

    // Right boundary.
    ((n - 3)..n).for_each(|i| {
        result[i] = vector[i] - 6.0 * vector[i - 1] + 15.0 * vector[i - 2] - 20.0 * vector[i - 3]
            + 15.0 * vector[i - 4]
            - 6.0 * vector[i - 5]
            + vector[i - 6];
    });

    return 1.0 / grid.delta / 64.0 * result;
}
