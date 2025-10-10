use crate::domain::grid::Grid;
use crate::domain::output_config::OutputConfig;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::state::State;

pub struct SimulationInputs {
    pub sim_config: SimulationConfig,
    pub out_config: OutputConfig,
    pub initial_state: State,
}

impl SimulationInputs {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.sim_config.grid.level < 5 {
            return Err("Level of discretization must be greater than 5.");
        }
        let num_points = Grid::length_at_discretization(self.sim_config.grid.level) as f64;
        if self.out_config.dt * num_points < 1.0 {
            return Err("Output dt is too short for the level of discretization. \
                 The output dt must be greater than 1 / 2^level_of_discretization.");
        }
        if self.out_config.dx_level > self.sim_config.grid.level {
            return Err(
                "Output dx level is greater than the level of discretization. \
                 The output dx level must be less than or equal to the level of discretization.",
            );
        }
        Ok(())
    }
}
