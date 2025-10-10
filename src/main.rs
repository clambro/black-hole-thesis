mod app;
mod domain;
mod use_cases;

use app::cli::Args;
use app::console_logger::ConsoleLogger;
use app::file_output::JsonlStateOutputCreator;
use use_cases::simulate::simulate;

fn main() {
    let inputs = Args::parse_args();

    let mut jsonl_output = JsonlStateOutputCreator::new(&inputs.sim_config);
    let logger = ConsoleLogger::new();

    simulate(&inputs, &mut jsonl_output, &logger);
}
