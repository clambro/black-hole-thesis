use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;
use crate::domain::parity::Parity;

/// Standard 4th order finite difference operator for the first derivative.
/// From https://web.media.mit.edu/~crtaylor/calculator.html
pub fn diff(
    grid: &Grid,
    vector: &FieldVector,
    parity_left: Parity,
    parity_right: Parity,
) -> FieldVector {
    let mut diff = FieldVector::zeros(vector.len());
    let n = vector.len();
    let h = grid.delta;
    let ghost_vec_left = get_ghost_vec(vector, parity_left);
    let ghost_vec_right = get_ghost_vec(vector, parity_right);

    // Left boundary.
    diff[0] =
        (ghost_vec_left[2] - 8.0 * ghost_vec_left[1] + 8.0 * vector[1] - vector[2]) / (12.0 * h);
    diff[1] = (ghost_vec_left[1] - 8.0 * vector[0] + 8.0 * vector[2] - vector[3]) / (12.0 * h);

    // Central difference for the interior points.
    (2..n - 2).for_each(|i| {
        diff[i] = (vector[i - 2] - 8.0 * vector[i - 1] + 8.0 * vector[i + 1] - vector[i + 2])
            / (12.0 * h);
    });

    // Right boundary.
    diff[n - 2] = (vector[n - 4] - 8.0 * vector[n - 3] + 8.0 * vector[n - 1]
        - ghost_vec_right[n - 2])
        / (12.0 * h);
    diff[n - 1] = (vector[n - 3] - 8.0 * vector[n - 2] + 8.0 * ghost_vec_right[n - 2]
        - ghost_vec_right[n - 3])
        / (12.0 * h);

    return diff;
}

/// Set the Neumann boundary condition for a vector using the above stencil.
/// The stencil here is weird, but it needs to be centered and 4th order accurate
/// and include the first/last point.
pub fn set_neumann_bc(vector: &mut FieldVector, left: bool, parity: Parity) {
    let ghost_vec = get_ghost_vec(vector, parity);
    if left {
        vector[0] = (3.0 * ghost_vec[2] - 30.0 * ghost_vec[1] + 60.0 * vector[1]
            - 15.0 * vector[2]
            + 2.0 * vector[3])
            / 20.0;
    } else {
        let n = vector.len();
        vector[n - 1] = (2.0 * vector[n - 4] - 15.0 * vector[n - 3] + 60.0 * vector[n - 2]
            - 30.0 * ghost_vec[n - 2]
            + 3.0 * ghost_vec[n - 3])
            / 20.0;
    }
}

/// Apply a 5th order Kreiss-Oliger dissipation operator.
/// This smooths out high frequency noise at the 5th order level without affecting our 4th order accuracy.
/// Stencils also from https://web.media.mit.edu/~crtaylor/calculator.html
pub fn dissipation(
    vector: &FieldVector,
    grid: &Grid,
    parity_left: Parity,
    parity_right: Parity,
) -> FieldVector {
    let mut result = FieldVector::zeros(vector.len());
    let n = vector.len();
    let ghost_vec_left = get_ghost_vec(vector, parity_left);
    let ghost_vec_right = get_ghost_vec(vector, parity_right);

    // Left boundary.
    result[0] = vector[3] - 6.0 * vector[2] + 15.0 * vector[1] - 20.0 * vector[0]
        + 15.0 * ghost_vec_left[1]
        - 6.0 * ghost_vec_left[2]
        + ghost_vec_left[3];

    result[1] = vector[4] - 6.0 * vector[3] + 15.0 * vector[2] - 20.0 * vector[1]
        + 15.0 * vector[0]
        - 6.0 * ghost_vec_left[1]
        + ghost_vec_left[2];

    result[2] = vector[5] - 6.0 * vector[4] + 15.0 * vector[3] - 20.0 * vector[2]
        + 15.0 * vector[1]
        - 6.0 * vector[0]
        + ghost_vec_left[1];

    // Interior points.
    (3..n - 3).for_each(|i| {
        result[i] = vector[i + 3] - 6.0 * vector[i + 2] + 15.0 * vector[i + 1] - 20.0 * vector[i]
            + 15.0 * vector[i - 1]
            - 6.0 * vector[i - 2]
            + vector[i - 3];
    });

    // Right boundary.
    result[n - 3] = ghost_vec_right[n - 2] - 6.0 * vector[n - 1] + 15.0 * vector[n - 2]
        - 20.0 * vector[n - 3]
        + 15.0 * vector[n - 4]
        - 6.0 * vector[n - 5]
        + vector[n - 6];
    result[n - 2] = ghost_vec_right[n - 3] - 6.0 * ghost_vec_right[n - 2] + 15.0 * vector[n - 1]
        - 20.0 * vector[n - 2]
        + 15.0 * vector[n - 3]
        - 6.0 * vector[n - 4]
        + vector[n - 5];
    result[n - 1] = ghost_vec_right[n - 4] - 6.0 * ghost_vec_right[n - 3]
        + 15.0 * ghost_vec_right[n - 2]
        - 20.0 * vector[n - 1]
        + 15.0 * vector[n - 2]
        - 6.0 * vector[n - 3]
        + vector[n - 4];

    return 1.0 / grid.delta / 64.0 * result;
}

fn get_ghost_vec(vector: &FieldVector, parity: Parity) -> FieldVector {
    match parity {
        Parity::Even => vector.clone(),
        Parity::Odd => -vector.clone(),
    }
}
