use clap::Parser;
use corkscrew::{builder, executor, parser, printer, settings::Settings, types::cli::Cli};
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Parse settings from CLI args
    let settings: Settings = Cli::parse()
        .try_into()
        .expect("Failed to parse provided arguments. Run 'corkscrew --help' for help.");

    // 2. Read in config file
    let contents = fs::read_to_string(&settings.config_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read configuration file from {}",
            &settings.config_path.display(),
        )
    });

    // 3. Parse config
    let details = parser::parse(&contents, settings.request_names);

    // 4. Build requests
    let requests = builder::build(details).await?;

    // 5. Execute requests
    let results = executor::exec(requests).await?;

    // 6. Display results
    printer::print(results);

    Ok(())
}
