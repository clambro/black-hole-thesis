use crate::domain::boundary_conditions::{BoundaryCondition, BoundaryConditions};
use crate::domain::config::Config;
use crate::use_cases::diff::diff;
use crate::use_cases::vector_math::{vec_add, vec_scalar_mul};
use std::ops::{Add, Mul};

pub struct EquationsOfMotion {
    pub d_dt_displacement: Vec<f64>,
    pub d_dt_momentum: Vec<f64>,
}

impl EquationsOfMotion {
    pub fn new(
        config: &Config,
        displacement: &Vec<f64>,
        momentum: &Vec<f64>,
    ) -> Self {
        let d_dt_displacement = Self::calculate_d_dt_displacement(momentum);
        let d_dt_momentum = Self::calculate_d_dt_momentum(config, displacement);
        Self {
            d_dt_displacement: d_dt_displacement,
            d_dt_momentum: d_dt_momentum,
        }
    }

    fn calculate_d_dt_displacement(momentum: &Vec<f64>) -> Vec<f64> {
        return momentum.clone();
    }

    fn calculate_d_dt_momentum(
        config: &Config,
        displacement: &Vec<f64>,
    ) -> Vec<f64> {
        let mut d_dt_momentum: Vec<f64> = displacement.clone();
        Self::apply_zero_bc(&mut d_dt_momentum, &config.boundary_conditions, BoundaryCondition::Dirichlet);

        d_dt_momentum = diff(&config.grid, &d_dt_momentum);
        Self::apply_zero_bc(&mut d_dt_momentum, &config.boundary_conditions, BoundaryCondition::Neumann);

        d_dt_momentum = diff(&config.grid, &d_dt_momentum);
        d_dt_momentum = d_dt_momentum
            .iter()
            .map(|x| config.wave_speed * config.wave_speed * x)
            .collect();
        return d_dt_momentum;
    }

    fn apply_zero_bc(vector: &mut Vec<f64>, bcs: &BoundaryConditions, bc_type: BoundaryCondition) {
        if bcs.left == bc_type {
            vector[0] = 0.0;
        }
        if bcs.right == bc_type {
            let length = vector.len();
            vector[length - 1] = 0.0;
        }
    }
}

impl Add for EquationsOfMotion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            d_dt_displacement: vec_add(&self.d_dt_displacement, &other.d_dt_displacement),
            d_dt_momentum: vec_add(&self.d_dt_momentum, &other.d_dt_momentum),
        }
    }
}

impl Mul<f64> for EquationsOfMotion {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            d_dt_displacement: vec_scalar_mul(scalar, &self.d_dt_displacement),
            d_dt_momentum: vec_scalar_mul(scalar, &self.d_dt_momentum),
        }
    }
}
