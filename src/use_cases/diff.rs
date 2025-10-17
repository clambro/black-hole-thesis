use crate::domain::constants::DISSIPATION_FACTOR;
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

    diff
}

/// Standard 4th order finite difference operator for the second derivative.
/// From https://web.media.mit.edu/~crtaylor/calculator.html
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

    diff2
}

/// Set the Neumann boundary condition for a vector using the 4th order stencil.
pub fn set_left_neumann_bc(vector: &mut FieldVector) {
    vector[0] = (48.0 * vector[1] - 36.0 * vector[2] + 16.0 * vector[3] - 3.0 * vector[4]) / 25.0;
}

/// Apply a 5th order Kreiss-Oliger dissipation operator with a radial correction.
/// This smooths out high frequency noise at the 5th order level without affecting our 4th order accuracy.
pub fn dissipation(vector: &FieldVector, grid: &Grid) -> FieldVector {
    let half_dissipation = diff3_unnormalized(vector);

    // High frequency noise is at the origin, so dampen the dissipation elsewhere.
    // To ensure positive semi-definiteness, the correction must be applied like
    // D(c*Du), not D^2(c*u), and not cD^2(u).
    let radial_correction = (1.0 - &grid.points.powi(2)).powi(4);

    let dissipation = diff3_unnormalized(&(radial_correction * half_dissipation));

    DISSIPATION_FACTOR / grid.delta / 64.0 * dissipation
}

/// Fourth order finite difference operator for the third derivative, without normalization by the grid spacing.
/// From https://web.media.mit.edu/~crtaylor/calculator.html
fn diff3_unnormalized(vector: &FieldVector) -> FieldVector {
    let mut diff3 = FieldVector::zeros(vector.len());
    let n = vector.len();

    // Left boundary.
    (0..3).for_each(|i| {
        diff3[i] = (vector[i] - 8.0 * vector[i + 1] + 13.0 * vector[i + 2] - 13.0 * vector[i + 4]
            + 8.0 * vector[i + 5]
            - vector[i + 6])
            / 8.0;
    });

    // Interior points.
    (3..n - 3).for_each(|i| {
        diff3[i] = (vector[i - 3] - 8.0 * vector[i - 2] + 13.0 * vector[i - 1]
            - 13.0 * vector[i + 1]
            + 8.0 * vector[i + 2]
            - vector[i + 3])
            / 8.0;
    });

    // Right boundary.
    (n - 3..n).for_each(|i| {
        diff3[i] = (vector[i - 6] - 8.0 * vector[i - 5] + 13.0 * vector[i - 4]
            - 13.0 * vector[i - 2]
            + 8.0 * vector[i - 1]
            - vector[i])
            / 8.0;
    });

    diff3
}
