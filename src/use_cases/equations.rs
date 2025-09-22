use crate::domain::boundary_conditions::{BoundaryCondition, BoundaryConditions};
use crate::domain::grid::Grid;
use crate::use_cases::diff::diff;
use crate::use_cases::vector_math::{vec_add, vec_scalar_mul};
use std::ops::{Add, Mul};

pub struct RightHandSide {
    pub position_dot: Vec<f64>,
    pub velocity_dot: Vec<f64>,
}

impl RightHandSide {
    pub fn new(
        grid: &Grid,
        wave_speed: f64,
        position: &Vec<f64>,
        velocity: &Vec<f64>,
        bcs: &BoundaryConditions,
    ) -> Self {
        let position_dot = Self::calculate_position_dot(velocity);
        let velocity_dot = Self::calculate_velocity_dot(grid, wave_speed, position, bcs);
        Self {
            position_dot,
            velocity_dot,
        }
    }

    fn calculate_position_dot(velocity: &Vec<f64>) -> Vec<f64> {
        return velocity.clone();
    }

    fn calculate_velocity_dot(
        grid: &Grid,
        wave_speed: f64,
        position: &Vec<f64>,
        bcs: &BoundaryConditions,
    ) -> Vec<f64> {
        let mut velocity_dot: Vec<f64> = position.clone();
        Self::apply_zero_bc(&mut velocity_dot, bcs, BoundaryCondition::Dirichlet);

        velocity_dot = diff(&grid, &velocity_dot);
        Self::apply_zero_bc(&mut velocity_dot, bcs, BoundaryCondition::Neumann);

        velocity_dot = diff(&grid, &velocity_dot);
        velocity_dot = velocity_dot
            .iter()
            .map(|x| wave_speed * wave_speed * x)
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

impl Add for RightHandSide {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            position_dot: vec_add(&self.position_dot, &other.position_dot),
            velocity_dot: vec_add(&self.velocity_dot, &other.velocity_dot),
        }
    }
}

impl Mul<f64> for RightHandSide {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            position_dot: vec_scalar_mul(scalar, &self.position_dot),
            velocity_dot: vec_scalar_mul(scalar, &self.velocity_dot),
        }
    }
}
