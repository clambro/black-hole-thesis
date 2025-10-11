//! File output handling for the black hole reflection simulation.

use crate::domain::output_config::OutputConfig;
use crate::domain::simulation_config::SimulationConfig;
use crate::domain::simulation_output::SimulationOutput;
use crate::domain::state::State;
use crate::use_cases::ports::StateOutputCreator;
use crate::use_cases::state_output_builder::build_state_output;
use std::fs::File;
use std::io::Write;

/// JSONL file output creator for simulation state and results.
pub struct JsonlStateOutputCreator {
    /// File handle for writing simulation state data in JSONL format.
    state_file: File,
    /// File handle for writing final simulation results in JSON format.
    results_file: File,
}

impl JsonlStateOutputCreator {
    /// Create a new JSONL output creator with the specified configuration.
    pub fn new(config: &SimulationConfig) -> Self {
        let folder = format!("results/{}_{}", config.initial_amplitude, config.grid.level);
        std::fs::create_dir_all(&folder).expect("Could not create folder");
        let state_file =
            File::create(format!("{}/states.jsonl", folder)).expect("Could not create file");
        let results_file =
            File::create(format!("{}/results.json", folder)).expect("Could not create file");
        Self {
            state_file,
            results_file,
        }
    }
}

impl StateOutputCreator for JsonlStateOutputCreator {
    /// Save the current simulation state to the output file.
    fn save_state(
        &mut self,
        state: &State,
        out_config: &OutputConfig,
        sim_config: &SimulationConfig,
    ) {
        let state_output = build_state_output(state, out_config, sim_config);
        let json_str = serde_json::to_string(&state_output).expect("Could not serialize state");
        writeln!(&self.state_file, "{}", json_str).expect("Could not write to file");
        self.state_file.flush().expect("Could not flush file");
    }

    /// Save the final simulation outputs to the output file.
    fn save_final_results(&mut self, output: &SimulationOutput) {
        let json_str = serde_json::to_string(&output).expect("Could not serialize output");
        writeln!(&self.results_file, "{}", json_str).expect("Could not write to file");
        self.results_file.flush().expect("Could not flush file");
    }
}
