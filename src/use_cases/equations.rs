use crate::domain::field_vector::FieldVector;
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
    ) -> Self {
        let mut d_dt_ingoing =
            Self::calculate_d_dt_ingoing(config, &ingoing, &outgoing, &constraints)
                + dissipation(&ingoing, &config.grid);
        let mut d_dt_outgoing =
            Self::calculate_d_dt_outgoing(config, &ingoing, &outgoing, &constraints)
                + dissipation(&outgoing, &config.grid);

        // The initial conditions satisfy the BCs, so as long as the updates satisfy them
        // as well and the update process is linear, they will remain satisfied.
        Self::apply_bcs(&mut d_dt_ingoing, &mut d_dt_outgoing);

        Self {
            d_dt_ingoing,
            d_dt_outgoing,
        }
    }

    pub fn apply_bcs(ingoing: &mut FieldVector, outgoing: &mut FieldVector) {
        // On the left we require ingoing = outgoing to maintain regularity at the origin.
        let avg_left = 0.5 * (ingoing[0] + outgoing[0]);
        ingoing[0] = avg_left;
        outgoing[0] = avg_left;

        // On the right we require ingoing = -outgoing to create the reflection.
        let n = ingoing.len();
        let avg_right = 0.5 * (ingoing[n - 1] - outgoing[n - 1]);
        ingoing[n - 1] = avg_right;
        outgoing[n - 1] = -avg_right;
    }

    fn calculate_d_dt_ingoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let flux = -0.5
            * (&constraints.char_speed * diff(&config.grid, &ingoing)
                + diff(&config.grid, &(&constraints.char_speed * ingoing)));
        // Limiting behaviour for the source comes from L'Hôpital's rule.
        // TODO: It's inefficient to calculate the entire derivative.
        let difference = outgoing - ingoing;
        let mut source = &constraints.char_speed * &difference / &config.grid.points;
        source[0] = constraints.char_speed[0] * diff(&config.grid, &difference)[0];

        return flux + source;
    }

    fn calculate_d_dt_outgoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let flux = 0.5
            * (&constraints.char_speed * diff(&config.grid, &outgoing)
                + diff(&config.grid, &(&constraints.char_speed * outgoing)));
        // Same BC logic here.
        let difference = outgoing - ingoing;
        let mut source = &constraints.char_speed * &difference / &config.grid.points;
        source[0] = constraints.char_speed[0] * diff(&config.grid, &difference)[0];

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
