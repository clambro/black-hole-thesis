use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::equations::EquationsOfMotion;

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(&config, state.displacement.clone(), state.momentum.clone());
    let u2 = EquationsOfMotion::new(
        &config,
        &state.displacement + 0.5 * time_step * &u1.d_dt_displacement,
        &state.momentum + 0.5 * time_step * &u1.d_dt_momentum,
    );
    let u3 = EquationsOfMotion::new(
        &config,
        &state.displacement + 0.5 * time_step * &u2.d_dt_displacement,
        &state.momentum + 0.5 * time_step * &u2.d_dt_momentum,
    );
    let u4 = EquationsOfMotion::new(
        &config,
        &state.displacement + time_step * &u3.d_dt_displacement,
        &state.momentum + time_step * &u3.d_dt_momentum,
    );
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);
    return State {
        displacement: &state.displacement + &rk4.d_dt_displacement,
        momentum: &state.momentum + &rk4.d_dt_momentum,
        time: state.time + time_step,
    };
}

/// Integrate a vector spatially, respecting the same SBP norm weights as the diff operator.
pub fn integrate(vector: &FieldVector, grid_size: f64) -> f64 {
    let n = vector.len();

    // SBP norm weights for Strand (1994) 4th order operator
    let mut total = 0.0;

    // Boundary weights (these are the diagonal entries of the norm matrix)
    total += (17.0 / 48.0) * vector[0];
    total += (59.0 / 48.0) * vector[1];
    total += (43.0 / 48.0) * vector[2];
    total += (49.0 / 48.0) * vector[3];

    // Interior points (standard weight of 1)
    for i in 4..n - 4 {
        total += vector[i];
    }

    // Right boundary weights (symmetric)
    total += (49.0 / 48.0) * vector[n - 4];
    total += (43.0 / 48.0) * vector[n - 3];
    total += (59.0 / 48.0) * vector[n - 2];
    total += (17.0 / 48.0) * vector[n - 1];

    return total * grid_size;
}
