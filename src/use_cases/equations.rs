use crate::domain::field_vector::FieldVector;
use crate::domain::parity::Parity;
use crate::domain::{config::Config, constraints::Constraints};
use crate::use_cases::diff::{diff, dissipation, set_neumann_bc};
use std::ops::{Add, Mul};

pub struct EquationsOfMotion {
    pub dt_radial_gradient: FieldVector,
    pub dt_conj_momentum: FieldVector,
}

impl EquationsOfMotion {
    pub fn new(
        config: &Config,
        radial_gradient: FieldVector,
        conj_momentum: FieldVector,
        constraints: &Constraints,
    ) -> Self {
        let dt_radial_gradient =
            Self::calculate_dt_radial_gradient(config, &conj_momentum, &constraints)
                + dissipation(&radial_gradient, &config.grid, Parity::Odd, Parity::Even);
        let dt_conj_momentum =
            Self::calculate_dt_conj_momentum(config, &radial_gradient, &constraints)
                + dissipation(&conj_momentum, &config.grid, Parity::Even, Parity::Odd);

        Self {
            dt_radial_gradient,
            dt_conj_momentum,
        }
    }

    pub fn apply_bcs(radial_gradient: &mut FieldVector, conj_momentum: &mut FieldVector) {
        // Coordinate singularity at the origin only requires smoothness.
        radial_gradient[0] = 0.0;
        set_neumann_bc(conj_momentum, true, Parity::Even);

        // Artificial reflection at the right boundary.
        set_neumann_bc(radial_gradient, false, Parity::Even);
        let n = conj_momentum.len();
        conj_momentum[n - 1] = 0.0;
    }

    fn calculate_dt_radial_gradient(
        config: &Config,
        conj_momentum: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let fun = &constraints.char_speed * conj_momentum;
        return diff(&config.grid, &fun, Parity::Even, Parity::Odd);
    }

    fn calculate_dt_conj_momentum(
        config: &Config,
        radial_gradient: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let r2 = config.grid.points.powi(2);
        let fun = &r2 * &constraints.char_speed * radial_gradient;
        let fun = diff(&config.grid, &fun, Parity::Odd, Parity::Even);
        let mut fun = fun / &r2;
        fun[0] = 0.0; // Coordinate singularity at the origin.
        return fun;
    }
}

impl Add<EquationsOfMotion> for EquationsOfMotion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dt_radial_gradient: &self.dt_radial_gradient + &other.dt_radial_gradient,
            dt_conj_momentum: &self.dt_conj_momentum + &other.dt_conj_momentum,
        }
    }
}

impl Add<&EquationsOfMotion> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn add(self, other: &EquationsOfMotion) -> EquationsOfMotion {
        EquationsOfMotion {
            dt_radial_gradient: &self.dt_radial_gradient + &other.dt_radial_gradient,
            dt_conj_momentum: &self.dt_conj_momentum + &other.dt_conj_momentum,
        }
    }
}

impl Mul<f64> for EquationsOfMotion {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            dt_radial_gradient: scalar * &self.dt_radial_gradient,
            dt_conj_momentum: scalar * &self.dt_conj_momentum,
        }
    }
}
impl Mul<f64> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn mul(self, scalar: f64) -> EquationsOfMotion {
        EquationsOfMotion {
            dt_radial_gradient: scalar * &self.dt_radial_gradient,
            dt_conj_momentum: scalar * &self.dt_conj_momentum,
        }
    }
}
