use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;

/// Standard 4th order finite difference operator for the first derivative.
/// From https://web.media.mit.edu/~crtaylor/calculator.html
pub fn diff(grid: &Grid, vector: &FieldVector) -> FieldVector {
    let mut diff = FieldVector::zeros(vector.len());
    let n = vector.len();
    let h = grid.delta;

    // Left boundary.
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

    // Right boundary.
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

/// Standard 4th order finite difference operator for the second derivative.
pub fn diff2(grid: &Grid, vector: &FieldVector) -> FieldVector {
    let mut diff2 = FieldVector::zeros(vector.len());
    let n = vector.len();
    let h2 = grid.delta * grid.delta;

    // Left boundary.
    diff2[0] = (45.0 * vector[0] - 154.0 * vector[1] + 214.0 * vector[2] - 156.0 * vector[3]
        + 61.0 * vector[4]
        - 10.0 * vector[5])
        / (12.0 * h2);

    diff2[1] = (10.0 * vector[0] - 15.0 * vector[1] - 4.0 * vector[2] + 14.0 * vector[3]
        - 6.0 * vector[4]
        + vector[5])
        / (12.0 * h2);

    // Interior points.
    (2..n - 2).for_each(|i| {
        diff2[i] = (-vector[i - 2] + 16.0 * vector[i - 1] - 30.0 * vector[i]
            + 16.0 * vector[i + 1]
            - vector[i + 2])
            / (12.0 * h2);
    });

    // Right boundary.
    diff2[n - 2] = (10.0 * vector[n - 1] - 15.0 * vector[n - 2] - 4.0 * vector[n - 3]
        + 14.0 * vector[n - 4]
        - 6.0 * vector[n - 5]
        + vector[n - 6])
        / (12.0 * h2);

    diff2[n - 1] = (45.0 * vector[n - 1] - 154.0 * vector[n - 2] + 214.0 * vector[n - 3]
        - 156.0 * vector[n - 4]
        + 61.0 * vector[n - 5]
        - 10.0 * vector[n - 6])
        / (12.0 * h2);

    return diff2;
}

/// Set the Neumann boundary condition for a vector using the above stencil.
pub fn set_left_neumann_bc(vector: &mut FieldVector) {
    vector[0] = (48.0 * vector[1] - 36.0 * vector[2] + 16.0 * vector[3] - 3.0 * vector[4]) / 25.0;
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

    return 0.01 / grid.delta / 64.0 * result;
}
