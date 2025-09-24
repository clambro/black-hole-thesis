/// Interfaces for the app files to inject dependencies into the use cases.
use crate::domain::state_output::StateOutput;

pub trait StateOutputCreator {
    fn save_state(&self, state: &StateOutput);
}
