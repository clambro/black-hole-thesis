use crate::domain::state::State;
use crate::use_cases::ports::StateOutput;
use std::fs::File;
use std::io::Write;

pub struct JsonlFileOutput {
    file: File,
}

impl JsonlFileOutput {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl StateOutput for JsonlFileOutput {
    fn save_state(&self, state: &State) {
        let displacement_str = format!(
            "[{}]",
            state
                .displacement
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        writeln!(
            &self.file,
            "{{\"time\":{},\"displacement\":{}}}",
            state.time, displacement_str
        )
        .expect("Could not write to file");
    }
}
