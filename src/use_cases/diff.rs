use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;

/// Standard 4th order finite difference operator for the first derivative.
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

    // Interior points.
    (2..n - 2).for_each(|i| {
        diff[i] = (-vector[i - 2] + 8.0 * vector[i - 1] - 8.0 * vector[i + 1] + vector[i + 2])
            / (12.0 * h);
    });

    // Right boundary.
    diff[n - 2] = (3.0 * vector[n - 1] + 10.0 * vector[n - 2] - 18.0 * vector[n - 3]
        + 6.0 * vector[n - 4]
        - vector[n - 5])
        / (12.0 * h);

    diff[n - 1] = (25.0 * vector[n - 1] - 48.0 * vector[n - 2] + 36.0 * vector[n - 3]
        - 16.0 * vector[n - 4]
        + 3.0 * vector[n - 5])
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
pub fn set_neumann_bc(vector: &mut FieldVector, left: bool) {
    if left {
        vector[0] =
            (48.0 * vector[1] - 36.0 * vector[2] + 16.0 * vector[3] - 3.0 * vector[4]) / 25.0;
    } else {
        let n = vector.len();
        vector[n - 1] = (48.0 * vector[n - 2] - 36.0 * vector[n - 3] + 16.0 * vector[n - 4]
            - 3.0 * vector[n - 5])
            / 25.0;
    }
}
