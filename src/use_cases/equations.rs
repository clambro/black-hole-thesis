use crate::domain::field_vector::FieldVector;
use crate::domain::{config::Config, constraints::Constraints};
use crate::use_cases::diff::{diff, dissipation};
use std::ops::{Add, Mul};

pub struct EquationsOfMotion {
    pub dt_ingoing: FieldVector,
    pub dt_outgoing: FieldVector,
    pub dt_alternate_mass: FieldVector,
}

impl EquationsOfMotion {
    pub fn new(
        config: &Config,
        ingoing: FieldVector,
        outgoing: FieldVector,
        constraints: &Constraints,
    ) -> Self {
        let dt_ingoing = Self::calculate_dt_ingoing(config, &ingoing, &outgoing, &constraints)
            + dissipation(&ingoing, &config.grid, &outgoing, &(-1.0 * &outgoing));
        let dt_outgoing = Self::calculate_dt_outgoing(config, &ingoing, &outgoing, &constraints)
            + dissipation(&outgoing, &config.grid, &ingoing, &(-1.0 * &ingoing));
        let dt_alternate_mass =
            Self::calculate_dt_alternate_mass(config, &ingoing, &outgoing, &constraints);

        Self {
            dt_ingoing,
            dt_outgoing,
            dt_alternate_mass,
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

    fn calculate_dt_ingoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let flux = -1.0
            * diff(
                &config.grid,
                &(&constraints.char_speed * ingoing),
                &(&constraints.char_speed * outgoing),
                &(-1.0 * &constraints.char_speed * outgoing),
            );
        // Source has a coordinate singularity at the origin, so we use L'Hôpital's rule.
        // TODO: Abstract this out so we're not calculating it twice. Also we don't need the full
        // stencil here, just the first point.
        let mut source = &constraints.char_speed / &config.grid.points * (outgoing - ingoing);
        source[0] = constraints.char_speed[0]
            * diff(
                &config.grid,
                &(outgoing - ingoing),
                &(-1.0 * (outgoing - ingoing)),
                &(outgoing - ingoing),
            )[0];
        return flux + source;
    }

    fn calculate_dt_outgoing(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let flux = diff(
            &config.grid,
            &(&constraints.char_speed * outgoing),
            &(&constraints.char_speed * ingoing),
            &(-1.0 * &constraints.char_speed * ingoing),
        );
        let mut source = &constraints.char_speed / &config.grid.points * (outgoing - ingoing);
        source[0] = constraints.char_speed[0]
            * diff(
                &config.grid,
                &(outgoing - ingoing),
                &(-1.0 * (outgoing - ingoing)),
                &(outgoing - ingoing),
            )[0];
        return flux + source;
    }

    fn calculate_dt_alternate_mass(
        config: &Config,
        ingoing: &FieldVector,
        outgoing: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        return 0.25 * &config.grid.points.powi(2) * &constraints.radial_factor.powi(2)
            / &constraints.lapse
            * (outgoing.powi(2) - ingoing.powi(2));
    }
}

impl Add<EquationsOfMotion> for EquationsOfMotion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dt_ingoing: &self.dt_ingoing + &other.dt_ingoing,
            dt_outgoing: &self.dt_outgoing + &other.dt_outgoing,
            dt_alternate_mass: &self.dt_alternate_mass + &other.dt_alternate_mass,
        }
    }
}

impl Add<&EquationsOfMotion> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn add(self, other: &EquationsOfMotion) -> EquationsOfMotion {
        EquationsOfMotion {
            dt_ingoing: &self.dt_ingoing + &other.dt_ingoing,
            dt_outgoing: &self.dt_outgoing + &other.dt_outgoing,
            dt_alternate_mass: &self.dt_alternate_mass + &other.dt_alternate_mass,
        }
    }
}

impl Mul<f64> for EquationsOfMotion {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            dt_ingoing: scalar * &self.dt_ingoing,
            dt_outgoing: scalar * &self.dt_outgoing,
            dt_alternate_mass: scalar * &self.dt_alternate_mass,
        }
    }
}

impl Mul<f64> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn mul(self, scalar: f64) -> EquationsOfMotion {
        EquationsOfMotion {
            dt_ingoing: scalar * &self.dt_ingoing,
            dt_outgoing: scalar * &self.dt_outgoing,
            dt_alternate_mass: scalar * &self.dt_alternate_mass,
        }
    }
}
