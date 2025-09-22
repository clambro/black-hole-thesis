use std::fs::File;
use std::io::Write;
use crate::use_cases::ports::StateOutput;
use crate::domain::state::State;


pub struct JsonlFileOutput {
    file: File,
}

impl JsonlFileOutput {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl StateOutput for JsonlFileOutput {
    fn save_state(&self, state: &State) {    let position_str = format!(
        "[{}]",
        state
            .wave_position
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    writeln!(&self.file, "{{\"time\":{},\"position\":{}}}", state.time, position_str)
        .expect("Could not write to file");
    }
}