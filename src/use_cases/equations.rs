use crate::domain::field_vector::FieldVector;
use crate::domain::parity::Parity;
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
        let d_dt_ingoing = Self::calculate_d_dt_ingoing(config, &ingoing, &outgoing, &constraints)
            + dissipation(&ingoing, &config.grid);
        let d_dt_outgoing =
            Self::calculate_d_dt_outgoing(config, &ingoing, &outgoing, &constraints)
                + dissipation(&outgoing, &config.grid);

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
        ingoing[n - 1] = -outgoing[n - 1];
    }

    fn calculate_d_dt_ingoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let xdw = &constraints.char_speed
            * diff(
                &config.grid,
                ingoing,
                Parity::Swap(outgoing.clone()),
                Parity::Swap(-outgoing.clone()),
            );
        let wdx = ingoing
            * diff(
                &config.grid,
                &constraints.char_speed,
                Parity::Even,
                Parity::Even,
            );
        let dxw = diff(
            &config.grid,
            &(&constraints.char_speed * ingoing),
            Parity::Swap(&constraints.char_speed * outgoing),
            Parity::Swap(-(&constraints.char_speed * outgoing)),
        );
        let flux = -0.5 * (xdw + wdx + dxw);
        // Limiting behaviour for the source comes from L'Hôpital's rule.
        // TODO: It's inefficient to calculate the entire derivative.
        let mut difference = outgoing - ingoing;
        difference[0] = 0.0;
        let mut source = &constraints.char_speed * &difference / &config.grid.points;
        let d_diff =
            &constraints.char_speed * diff(&config.grid, &difference, Parity::Odd, Parity::Even);
        source[0] = d_diff[0];
        source[1] = 0.75 * d_diff[1] + 0.25 * source[1];
        source[2] = 0.5 * d_diff[2] + 0.5 * source[2];
        source[3] = 0.25 * d_diff[3] + 0.75 * source[3];

        return flux + source;
    }

    fn calculate_d_dt_outgoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let xdw = &constraints.char_speed
            * diff(
                &config.grid,
                outgoing,
                Parity::Swap(ingoing.clone()),
                Parity::Swap(-ingoing.clone()),
            );
        let wdx = outgoing
            * diff(
                &config.grid,
                &constraints.char_speed,
                Parity::Even,
                Parity::Even,
            );
        let dxw = diff(
            &config.grid,
            &(&constraints.char_speed * outgoing),
            Parity::Swap(&constraints.char_speed * ingoing),
            Parity::Swap(-(&constraints.char_speed * ingoing)),
        );
        let flux = 0.5 * (xdw + wdx + dxw);
        // Same BC logic here.
        let mut difference = outgoing - ingoing;
        difference[0] = 0.0;
        let mut source = &constraints.char_speed * &difference / &config.grid.points;
        let limit =
            &constraints.char_speed * diff(&config.grid, &difference, Parity::Odd, Parity::Even);
        source[0] = limit[0];
        source[1] = 0.75 * limit[1] + 0.25 * source[1];
        source[2] = 0.5 * limit[2] + 0.5 * source[2];
        source[3] = 0.25 * limit[3] + 0.75 * source[3];

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
