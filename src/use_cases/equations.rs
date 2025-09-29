use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::state::State;
use crate::use_cases::diff::{diff, diff2};
use std::ops::{Add, Mul};

pub struct EquationsOfMotion {
    pub d_dt_displacement: FieldVector,
    pub d_dt_momentum: FieldVector,
}

impl EquationsOfMotion {
    pub fn new(config: &Config, displacement: FieldVector, momentum: FieldVector) -> Self {
        let d_dt_displacement = Self::calculate_d_dt_displacement(&momentum);
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

    fn calculate_d_dt_displacement(momentum: &FieldVector) -> FieldVector {
        return momentum.clone();
    }

    fn calculate_d_dt_momentum(config: &Config, displacement: &FieldVector) -> FieldVector {
        let d_dt_momentum = diff2(&config.grid, displacement);
        return config.wave_speed.powi(2) * d_dt_momentum;
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
