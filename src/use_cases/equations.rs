use crate::domain::boundary_conditions::{BoundaryCondition, BoundaryConditions};
use crate::domain::config::Config;
use crate::use_cases::diff::diff;
use crate::use_cases::vector_math::{vec_add, vec_scalar_mul};
use std::ops::{Add, Mul};

pub struct EquationsOfMotion {
    pub position_dot: Vec<f64>,
    pub velocity_dot: Vec<f64>,
}

impl EquationsOfMotion {
    pub fn new(
        config: &Config,
        position: &Vec<f64>,
        velocity: &Vec<f64>,
    ) -> Self {
        let position_dot = Self::calculate_position_dot(velocity);
        let velocity_dot = Self::calculate_velocity_dot(config, position);
        Self {
            position_dot,
            velocity_dot,
        }
    }

    fn calculate_position_dot(velocity: &Vec<f64>) -> Vec<f64> {
        return velocity.clone();
    }

    fn calculate_velocity_dot(
        config: &Config,
        position: &Vec<f64>,
    ) -> Vec<f64> {
        let mut velocity_dot: Vec<f64> = position.clone();
        Self::apply_zero_bc(&mut velocity_dot, &config.boundary_conditions, BoundaryCondition::Dirichlet);

        velocity_dot = diff(&config.grid, &velocity_dot);
        Self::apply_zero_bc(&mut velocity_dot, &config.boundary_conditions, BoundaryCondition::Neumann);

        velocity_dot = diff(&config.grid, &velocity_dot);
        velocity_dot = velocity_dot
            .iter()
            .map(|x| config.wave_speed * config.wave_speed * x)
            .collect();
        return velocity_dot;
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
            position_dot: vec_add(&self.position_dot, &other.position_dot),
            velocity_dot: vec_add(&self.velocity_dot, &other.velocity_dot),
        }
    }
}

impl Mul<f64> for EquationsOfMotion {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            position_dot: vec_scalar_mul(scalar, &self.position_dot),
            velocity_dot: vec_scalar_mul(scalar, &self.velocity_dot),
        }
    }
}
