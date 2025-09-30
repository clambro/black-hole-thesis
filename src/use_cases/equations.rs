use crate::domain::field_vector::FieldVector;
use crate::domain::parity::Parity;
use crate::domain::state::State;
use crate::domain::{config::Config, constraints::Constraints};
use crate::use_cases::diff::{diff, dissipation};
use std::ops::{Add, Mul};

pub struct EquationsOfMotion {
    pub d_dt_ingoing: FieldVector,
    pub d_dt_outgoing: FieldVector,
}

impl EquationsOfMotion {
    pub fn new(
        config: &Config,
        ingoing: FieldVector,
        outgoing: FieldVector,
        constraints: &Constraints,
        time_step: f64,
    ) -> Self {
        let d_dt_ingoing = Self::calculate_d_dt_ingoing(config, &ingoing, &outgoing, &constraints);
        let d_dt_outgoing =
            Self::calculate_d_dt_outgoing(config, &ingoing, &outgoing, &constraints);
        Self {
            d_dt_ingoing: d_dt_ingoing + dissipation(&ingoing, &config.grid, time_step),
            d_dt_outgoing: d_dt_outgoing + dissipation(&outgoing, &config.grid, time_step),
        }
    }

    pub fn apply_bcs(ingoing: &mut FieldVector, outgoing: &mut FieldVector) {
        // On the left we require ingoing = outgoing to maintain regularity at the origin.
        let left_bc = 0.5 * (ingoing[0] + outgoing[0]);
        ingoing[0] = left_bc;
        outgoing[0] = left_bc;

        // On the right we require ingoing = -outgoing to create the reflection.
        let n = ingoing.len();
        let right_bc = 0.5 * (ingoing[n - 1] - outgoing[n - 1]);
        ingoing[n - 1] = right_bc;
        outgoing[n - 1] = -right_bc;
    }

    pub fn calculate_energy_density(state: &State) -> FieldVector {
        return 0.25 * (&state.ingoing.powi(2) + &state.outgoing.powi(2));
    }

    fn calculate_d_dt_ingoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let flux = diff(
            &config.grid,
            &(&constraints.char_speed * ingoing),
            Parity::Swap(&constraints.char_speed * outgoing),
        );
        // Limiting behaviour for the source comes from L'Hôpital's rule.
        // TODO: It's inefficient to calculate the difference twice and the entire derivative.
        let mut source = &constraints.char_speed / &config.grid.points * (ingoing - outgoing);
        source[0] =
            constraints.char_speed[0] * diff(&config.grid, &(ingoing - outgoing), Parity::Odd)[0];

        return flux + source;
    }

    fn calculate_d_dt_outgoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let flux = -diff(
            &config.grid,
            &(&constraints.char_speed * outgoing),
            Parity::Swap(&constraints.char_speed * ingoing),
        );
        // Same BC logic here.
        let mut source = &constraints.char_speed / &config.grid.points * (ingoing - outgoing);
        source[0] =
            constraints.char_speed[0] * diff(&config.grid, &(ingoing - outgoing), Parity::Odd)[0];

        return flux + source;
    }
}

impl Add<EquationsOfMotion> for EquationsOfMotion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            d_dt_ingoing: &self.d_dt_ingoing + &other.d_dt_ingoing,
            d_dt_outgoing: &self.d_dt_outgoing + &other.d_dt_outgoing,
        }
    }
}

impl Add<&EquationsOfMotion> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn add(self, other: &EquationsOfMotion) -> EquationsOfMotion {
        EquationsOfMotion {
            d_dt_ingoing: &self.d_dt_ingoing + &other.d_dt_ingoing,
            d_dt_outgoing: &self.d_dt_outgoing + &other.d_dt_outgoing,
        }
    }
}

impl Mul<f64> for EquationsOfMotion {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            d_dt_ingoing: scalar * &self.d_dt_ingoing,
            d_dt_outgoing: scalar * &self.d_dt_outgoing,
        }
    }
}
impl Mul<f64> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn mul(self, scalar: f64) -> EquationsOfMotion {
        EquationsOfMotion {
            d_dt_ingoing: scalar * &self.d_dt_ingoing,
            d_dt_outgoing: scalar * &self.d_dt_outgoing,
        }
    }
}
