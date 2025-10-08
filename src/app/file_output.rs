use crate::domain::config::Config;
use crate::domain::state::State;
use crate::domain::state_output::StateOutput;
use crate::use_cases::ports::StateOutputCreator;
use std::fs::File;
use std::io::Write;

pub struct JsonlStateOutputCreator {
    file: File,
}

impl JsonlStateOutputCreator {
    pub fn new(config: &Config) -> Self {
        let folder = format!("results/{}_{}", config.initial_amplitude, config.grid.level);
        std::fs::create_dir_all(&folder).expect("Could not create folder");
        let file = File::create(format!("{}/states.jsonl", folder)).expect("Could not create file");
        Self { file }
    }
}

impl StateOutputCreator for JsonlStateOutputCreator {
    fn save_state(&self, state: &State, config: &Config) {
        let state_output = StateOutput::from_state(state, config);
        let json_str = serde_json::to_string(&state_output).expect("Could not serialize state");
        writeln!(&self.file, "{}", json_str).expect("Could not write to file");
    }
}
