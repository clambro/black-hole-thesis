use crate::domain::field_vector::FieldVector;
use crate::domain::grid::Grid;
use crate::domain::parity::Parity;

/// Standard 4th order finite difference operator for the first derivative.
pub fn diff(grid: &Grid, vector: &FieldVector, parity: Parity) -> FieldVector {
    let mut diff = FieldVector::zeros(vector.len());
    let n = vector.len();
    let h = grid.delta;

    // Left side is calculated via parity. There's no physical boundary at r=0.
    match parity {
        Parity::Even => {
            diff[0] = 0.0;
            diff[1] = (vector[1] - 8.0 * vector[0] + 8.0 * vector[2] - vector[3]) / (12.0 * h);
        }
        Parity::Odd => {
            diff[0] = (8.0 * vector[1] - vector[2]) / (6.0 * h);
            diff[1] = (-vector[1] + 8.0 * vector[2] - vector[3]) / (12.0 * h); // f(0) = 0 for odd.
        }
        Parity::Swap(swap_vec) => {
            diff[0] = (swap_vec[2] - 8.0 * swap_vec[1] + 8.0 * vector[1] - vector[2]) / (12.0 * h);
            diff[1] = (swap_vec[1] - 8.0 * vector[0] + 8.0 * vector[2] - vector[3]) / (12.0 * h);
        }
    }

    // Interior points calculated via standard 4th order finite difference.
    (2..n - 2).for_each(|i| {
        diff[i] = (vector[i - 2] - 8.0 * vector[i - 1] + 8.0 * vector[i + 1] - vector[i + 2])
            / (12.0 * h);
    });

    // Right boundary calculated via backward difference.
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

/// Apply a 5th order Kreiss-Oliger dissipation operator: (1/64) * (dt/dx) * S(f),
/// where S(f) is the 6th order finite difference of f. Note that S does not include the dx^6 term.
/// This smooths out high frequency noise at the 5th order level without affecting our 4th order accuracy.
pub fn dissipation(vector: &FieldVector, grid: &Grid, time_step: f64) -> FieldVector {
    let mut result = FieldVector::zeros(vector.len());
    return result;
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

    return time_step / grid.delta / 64.0 * result;
}
