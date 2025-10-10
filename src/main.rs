mod app;
mod domain;
mod use_cases;

use app::cli::Args;
use app::console_logger::ConsoleLogger;
use app::file_output::JsonlStateOutputCreator;
use use_cases::simulate::simulate;

fn main() {
    let (config, state) = Args::parse_args();

    let jsonl_output = JsonlStateOutputCreator::new(&config);
    let logger = ConsoleLogger::new();

    simulate(&config, state, &jsonl_output, &logger);
}
