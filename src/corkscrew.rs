use crate::{config::Config, exec::Exec, load::Load, parse::Parse, requests::Requests};

pub fn run(config: Config) {
    Requests::new(config)
        .load()
        .expect("Failed to load requests file. See error(s) above for details.")
        .parse()
        .expect("Failed to parse requests. See error(s) above for details.")
        .exec()
        .expect("Failed to execute requests. See error(s) above for details.");
}
