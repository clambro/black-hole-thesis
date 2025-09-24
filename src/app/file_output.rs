use crate::domain::state::State;
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
