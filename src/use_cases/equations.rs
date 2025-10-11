use crate::domain::field_vector::FieldVector;
use crate::domain::{constraints::Constraints, simulation_config::SimulationConfig};
use crate::use_cases::diff::{diff, diff2, dissipation, set_left_neumann_bc};
use std::ops::{Add, Mul};

/// Time derivatives of the field variables.
pub struct EquationsOfMotion {
    /// Time derivative of the scalar field.
    pub dt_field: FieldVector,
    /// Time derivative of the conjugate momentum.
    pub dt_conj_momentum: FieldVector,
    /// Time derivative of the alternate mass function.
    pub dt_alternate_mass: FieldVector,
}

impl EquationsOfMotion {
    /// Create equations of motion from current state.
    pub fn new(
        config: &SimulationConfig,
        field: &FieldVector,
        conj_momentum: &FieldVector,
        constraints: &Constraints,
    ) -> Self {
        let dt_field =
            Self::calculate_dt_field(conj_momentum, constraints) + dissipation(field, &config.grid);
        let dt_conj_momentum = Self::calculate_dt_conj_momentum(config, field, constraints)
            + dissipation(conj_momentum, &config.grid);
        // The mass function is smooth, so no dissipation required.
        let dt_alternate_mass =
            Self::calculate_dt_alternate_mass(config, field, conj_momentum, constraints);

        Self {
            dt_field,
            dt_conj_momentum,
            dt_alternate_mass,
        }
    }

    /// Apply boundary conditions to the fields.
    pub fn apply_bcs(field: &mut FieldVector, conj_momentum: &mut FieldVector) {
        // Neumann BCs at the origin maintain regularity.
        set_left_neumann_bc(field);
        set_left_neumann_bc(conj_momentum);

        // On the right we have a Dirichlet BC to create the reflection.
        let n = field.len();
        field[n - 1] = 0.0;
        conj_momentum[n - 1] = 0.0;
    }

    /// Calculate the time derivative of the scalar field.
    fn calculate_dt_field(conj_momentum: &FieldVector, constraints: &Constraints) -> FieldVector {
        let mut result = &constraints.char_speed * conj_momentum;
        result[0] = conj_momentum[0] / constraints.lapse[0]; // L'Hopital's rule.
        result
    }

    /// Calculate the time derivative of the conjugate momentum.
    fn calculate_dt_conj_momentum(
        config: &SimulationConfig,
        field: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let d2_field = diff2(&config.grid, field);
        let curvature = &constraints.char_speed * &d2_field;
        let divergence = (&constraints.radial_factor + 1.0)
            / (&config.grid.points * &constraints.lapse)
            * diff(&config.grid, field);
        let mut result = curvature + divergence;
        result[0] = 3.0 * &d2_field[0] / constraints.lapse[0]; // L'Hopital's rule.
        result
    }

    /// Calculate the time derivative of the alternate mass function.
    fn calculate_dt_alternate_mass(
        config: &SimulationConfig,
        field: &FieldVector,
        conj_momentum: &FieldVector,
        constraints: &Constraints,
    ) -> FieldVector {
        let radial_gradient = diff(&config.grid, field);
        &config.grid.points.powi(2) * &constraints.radial_factor.powi(2) / &constraints.lapse
            * &radial_gradient
            * conj_momentum
    }
}

impl Add<EquationsOfMotion> for EquationsOfMotion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dt_field: &self.dt_field + &other.dt_field,
            dt_conj_momentum: &self.dt_conj_momentum + &other.dt_conj_momentum,
            dt_alternate_mass: &self.dt_alternate_mass + &other.dt_alternate_mass,
        }
    }
}

impl Add<&EquationsOfMotion> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn add(self, other: &EquationsOfMotion) -> EquationsOfMotion {
        EquationsOfMotion {
            dt_field: &self.dt_field + &other.dt_field,
            dt_conj_momentum: &self.dt_conj_momentum + &other.dt_conj_momentum,
            dt_alternate_mass: &self.dt_alternate_mass + &other.dt_alternate_mass,
        }
    }
}

impl Mul<f64> for EquationsOfMotion {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            dt_field: scalar * &self.dt_field,
            dt_conj_momentum: scalar * &self.dt_conj_momentum,
            dt_alternate_mass: scalar * &self.dt_alternate_mass,
        }
    }
}

impl Mul<f64> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn mul(self, scalar: f64) -> EquationsOfMotion {
        EquationsOfMotion {
            dt_field: scalar * &self.dt_field,
            dt_conj_momentum: scalar * &self.dt_conj_momentum,
            dt_alternate_mass: scalar * &self.dt_alternate_mass,
        }
    }
}
