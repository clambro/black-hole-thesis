use crate::domain::state_output::StateOutput;
use crate::use_cases::ports::StateOutputCreator;
use std::fs::File;
use std::io::Write;

pub struct JsonlStateOutputCreator {
    file: File,
}

impl JsonlStateOutputCreator {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl StateOutputCreator for JsonlStateOutputCreator {
    fn save_state(&self, state_output: &StateOutput) {
        let json_str = serde_json::to_string(state_output).expect("Could not serialize state");
        writeln!(&self.file, "{}", json_str).expect("Could not write to file");
    }
}
