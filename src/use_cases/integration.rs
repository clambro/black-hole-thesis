use crate::domain::boundary_conditions::{BoundaryCondition, BoundaryConditions};
use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::diff::set_neumann_bc;
use crate::use_cases::equations::EquationsOfMotion;

/// Perform a Runge-Kutta 4th order time step.
pub fn rk4_step(config: &Config, state: &State, time_step: f64) -> State {
    let u1 = EquationsOfMotion::new(&config, state.displacement.clone(), state.momentum.clone());
    let mut u1_displacement = &state.displacement + 0.5 * time_step * &u1.d_dt_displacement;
    let mut u1_momentum = &state.momentum + 0.5 * time_step * &u1.d_dt_momentum;
    // This BC logic is a bit ugly, but we're going to change it dramatically for the black hole stuff,
    // so no point in refactoring it right now.
    apply_bcs(&mut u1_displacement, &config.boundary_conditions);
    apply_bcs(&mut u1_momentum, &config.boundary_conditions);

    let u2 = EquationsOfMotion::new(&config, u1_displacement, u1_momentum);
    let mut u2_displacement = &state.displacement + 0.5 * time_step * &u2.d_dt_displacement;
    let mut u2_momentum = &state.momentum + 0.5 * time_step * &u2.d_dt_momentum;
    apply_bcs(&mut u2_displacement, &config.boundary_conditions);
    apply_bcs(&mut u2_momentum, &config.boundary_conditions);

    let u3 = EquationsOfMotion::new(&config, u2_displacement, u2_momentum);
    let mut u3_displacement = &state.displacement + time_step * &u3.d_dt_displacement;
    let mut u3_momentum = &state.momentum + time_step * &u3.d_dt_momentum;
    apply_bcs(&mut u3_displacement, &config.boundary_conditions);
    apply_bcs(&mut u3_momentum, &config.boundary_conditions);

    let u4 = EquationsOfMotion::new(&config, u3_displacement, u3_momentum);
    let rk4: EquationsOfMotion = (u1 + u2 * 2.0 + u3 * 2.0 + u4) * (time_step / 6.0);

    let mut displacement = &state.displacement + &rk4.d_dt_displacement;
    let mut momentum = &state.momentum + &rk4.d_dt_momentum;
    apply_bcs(&mut displacement, &config.boundary_conditions);
    apply_bcs(&mut momentum, &config.boundary_conditions);

    return State {
        displacement,
        momentum,
        time: state.time + time_step,
    };
}

/// Integrate a vector spatially using Simpson's rule (4th order accurate).
pub fn integrate(vector: &FieldVector, grid_size: f64) -> f64 {
    let n = vector.len();
    let mut sum = vector[0] + vector[n - 1];

    for i in 1..n - 1 {
        if i % 2 == 1 {
            sum += 4.0 * vector[i];
        } else {
            sum += 2.0 * vector[i];
        }
    }

    return sum * grid_size / 3.0;
}

fn apply_bcs(vector: &mut FieldVector, bcs: &BoundaryConditions) {
    if bcs.left == BoundaryCondition::Dirichlet {
        vector[0] = 0.0;
    } else if bcs.left == BoundaryCondition::Neumann {
        set_neumann_bc(vector, true);
    }
    if bcs.right == BoundaryCondition::Dirichlet {
        let length = vector.len();
        vector[length - 1] = 0.0;
    } else if bcs.right == BoundaryCondition::Neumann {
        set_neumann_bc(vector, false);
    }
}
