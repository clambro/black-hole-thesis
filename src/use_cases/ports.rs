/// Interfaces for the app files to inject dependencies into the use cases.
use crate::domain::config::Config;
use crate::domain::state::State;

pub trait StateOutputCreator {
    fn save_state(&self, state: &State, config: &Config);
}
