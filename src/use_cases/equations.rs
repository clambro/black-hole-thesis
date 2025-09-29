use crate::domain::boundary_conditions::{BoundaryCondition, BoundaryConditions};
use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::diff::diff;
use std::ops::{Add, Mul};

pub struct EquationsOfMotion {
    pub d_dt_displacement: FieldVector,
    pub d_dt_momentum: FieldVector,
}

impl EquationsOfMotion {
    pub fn new(config: &Config, displacement: FieldVector, momentum: FieldVector) -> Self {
        let d_dt_displacement = Self::calculate_d_dt_displacement(config, &momentum);
        let d_dt_momentum = Self::calculate_d_dt_momentum(config, &displacement);
        Self {
            d_dt_displacement,
            d_dt_momentum,
        }
    }

    pub fn calculate_energy_density(state: &State, config: &Config) -> FieldVector {
        let kinetic_energy = 0.5 * state.momentum.clone().powi(2);

        let du_dx = diff(&config.grid, &state.displacement);
        let potential_energy = 0.5 * du_dx.powi(2) * config.wave_speed.powi(2);

        return kinetic_energy + potential_energy;
    }

    fn calculate_d_dt_displacement(config: &Config, momentum: &FieldVector) -> FieldVector {
        let mut d_dt_displacement = momentum.clone();
        Self::apply_zero_bc(
            &mut d_dt_displacement,
            &config.boundary_conditions,
            BoundaryCondition::Dirichlet,
        );
        return d_dt_displacement;
    }

    fn calculate_d_dt_momentum(config: &Config, displacement: &FieldVector) -> FieldVector {
        let mut d_dt_momentum = displacement.clone();
        Self::apply_zero_bc(
            &mut d_dt_momentum,
            &config.boundary_conditions,
            BoundaryCondition::Dirichlet,
        );

        d_dt_momentum = diff(&config.grid, &d_dt_momentum);
        Self::apply_zero_bc(
            &mut d_dt_momentum,
            &config.boundary_conditions,
            BoundaryCondition::Neumann,
        );

        d_dt_momentum = diff(&config.grid, &d_dt_momentum);
        d_dt_momentum = config.wave_speed.powi(2) * d_dt_momentum;
        return d_dt_momentum;
    }

    fn apply_zero_bc(
        vector: &mut FieldVector,
        bcs: &BoundaryConditions,
        bc_type: BoundaryCondition,
    ) {
        if bcs.left == bc_type {
            vector[0] = 0.0;
        }
        if bcs.right == bc_type {
            let length = vector.len();
            vector[length - 1] = 0.0;
        }
    }
}

impl Add<EquationsOfMotion> for EquationsOfMotion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            d_dt_displacement: &self.d_dt_displacement + &other.d_dt_displacement,
            d_dt_momentum: &self.d_dt_momentum + &other.d_dt_momentum,
        }
    }
}

impl Add<&EquationsOfMotion> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn add(self, other: &EquationsOfMotion) -> EquationsOfMotion {
        EquationsOfMotion {
            d_dt_displacement: &self.d_dt_displacement + &other.d_dt_displacement,
            d_dt_momentum: &self.d_dt_momentum + &other.d_dt_momentum,
        }
    }
}

impl Mul<f64> for EquationsOfMotion {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            d_dt_displacement: scalar * &self.d_dt_displacement,
            d_dt_momentum: scalar * &self.d_dt_momentum,
        }
    }
}
impl Mul<f64> for &EquationsOfMotion {
    type Output = EquationsOfMotion;

    fn mul(self, scalar: f64) -> EquationsOfMotion {
        EquationsOfMotion {
            d_dt_displacement: scalar * &self.d_dt_displacement,
            d_dt_momentum: scalar * &self.d_dt_momentum,
        }
    }
}
