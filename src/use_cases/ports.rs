/// Interfaces for the app files to inject dependencies into the use cases.

use crate::domain::state::State;

pub trait StateOutput {
    fn save_state(&self, state: &State, time: f64);  // TODO: Time needs to be added to the state.
}