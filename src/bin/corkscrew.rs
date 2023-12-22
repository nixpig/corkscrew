use clap::Parser;
use cli_table::{print_stdout, WithTitle};
use corkscrew::cli::Cli;
use corkscrew::parser;
use corkscrew::{error::CorkscrewError, settings::Settings};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), CorkscrewError> {
    let settings: Settings = Cli::parse()
        .try_into()
        .expect("Failed to parse provided arguments. Run 'corkscrew --help' for help.");

    let contents = fs::read_to_string(&settings.config_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read configuration file from {}",
            &settings.config_path.display(),
        )
    });

    let config = parser::parse(&settings, &contents);

    // 4. Build requests
    // let requests = builder::build(&config);

    // 5. Execute requests
    // let results = executor::execute(&requests);
    let results = corkscrew::requests::Requests::exec(&config)
        .await
        .expect("Failed to execute requests.");

    // 6. Display results
    // let _ = displayer::display(&results);
    let _ = print_stdout(results.with_title());

    Ok(())
}
