/// Interfaces for the app files to inject dependencies into the use cases.
use crate::domain::config::Config;
use crate::domain::simulation_output::SimulationOutput;
use crate::domain::state::State;

pub trait StateOutputCreator {
    fn save_state(&self, state: &State, config: &Config);
    fn save_final_results(&self, output: &SimulationOutput);
}

pub trait SimulationLogger {
    fn log_progress(&self, elapsed_seconds: f64, num_steps: i32, simulation_time: f64);
    fn log_timeout_warning(&self, max_time: f64);
    fn log_final_results(&self, output: &SimulationOutput);
}
