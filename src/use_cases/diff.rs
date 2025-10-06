use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;
use crate::domain::parity::Parity;

/// Standard 4th order finite difference operator for the first derivative.
/// From https://web.media.mit.edu/~crtaylor/calculator.html
pub fn diff(
    grid: &Grid,
    vector: &FieldVector,
    left_parity: Parity,
    right_parity: Parity,
) -> FieldVector {
    let mut diff = FieldVector::zeros(vector.len());
    let n = vector.len();
    let h = grid.delta;

    // Left boundary.
    match left_parity {
        Parity::Even => {
            diff[0] = 0.0;
            diff[1] = (vector[1] - 8.0 * vector[0] + 8.0 * vector[2] - vector[3]) / (12.0 * h);
        }
        Parity::Odd => {
            diff[0] = (8.0 * vector[1] - vector[2]) / (6.0 * h);
            diff[1] = (-vector[1] - 8.0 * vector[0] + 8.0 * vector[2] - vector[3]) / (12.0 * h);
        }
        Parity::Swap(swap_vector) => {
            diff[0] =
                (swap_vector[2] - 8.0 * swap_vector[1] + 8.0 * vector[1] - vector[2]) / (12.0 * h);
            diff[1] = (swap_vector[1] - 8.0 * vector[0] + 8.0 * vector[2] - vector[3]) / (12.0 * h);
        }
    }

    // Central difference for the interior points.
    (2..n - 2).for_each(|i| {
        diff[i] = (vector[i - 2] - 8.0 * vector[i - 1] + 8.0 * vector[i + 1] - vector[i + 2])
            / (12.0 * h);
    });

    // Right boundary.
    match right_parity {
        Parity::Even => {
            diff[n - 2] = (vector[n - 4] - 8.0 * vector[n - 3] + 8.0 * vector[n - 1]
                - vector[n - 2])
                / (12.0 * h);
            diff[n - 1] = 0.0;
        }
        Parity::Odd => {
            diff[n - 2] =
                (vector[n - 4] - 8.0 * vector[n - 3] + 8.0 * vector[n - 1] + vector[n - 2])
                    / (12.0 * h);
            diff[n - 1] = (vector[n - 3] - 8.0 * vector[n - 2]) / (6.0 * h);
        }
        Parity::Swap(swap_vector) => {
            diff[n - 2] = (vector[n - 4] - 8.0 * vector[n - 3] + 8.0 * vector[n - 1]
                - swap_vector[n - 2])
                / (12.0 * h);
            diff[n - 1] = (vector[n - 3] - 8.0 * vector[n - 2] + 8.0 * swap_vector[n - 2]
                - swap_vector[n - 3])
                / (12.0 * h);
        }
    }

    return diff;
}

/// Apply a 5th order Kreiss-Oliger dissipation operator.
/// This smooths out high frequency noise at the 5th order level without affecting our 4th order accuracy.
/// Stencils also from https://web.media.mit.edu/~crtaylor/calculator.html
pub fn dissipation(vector: &FieldVector, grid: &Grid) -> FieldVector {
    let mut result = FieldVector::zeros(vector.len());
    let n = vector.len();

    // Left boundary. Skip i=0 since it's constrained by the BC.
    (1..3).for_each(|i| {
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

    // Right boundary. Skip i=n-1 since it's constrained by the BC.
    ((n - 3)..(n - 1)).for_each(|i| {
        result[i] = vector[i] - 6.0 * vector[i - 1] + 15.0 * vector[i - 2] - 20.0 * vector[i - 3]
            + 15.0 * vector[i - 4]
            - 6.0 * vector[i - 5]
            + vector[i - 6];
    });

    return 1.0 / grid.delta / 64.0 * result;
}
