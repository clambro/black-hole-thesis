/// Interfaces for the app files to inject dependencies into the use cases.

use crate::domain::state::State;

pub trait StateOutput {
    fn save_state(&self, state: &State);
}