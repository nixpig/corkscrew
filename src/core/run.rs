use std::{collections::HashMap, error::Error, fs};

use crate::{builder, executor, parser, Settings};

pub async fn go(
    settings: Settings,
) -> Result<HashMap<String, Box<reqwest::Response>>, Box<dyn Error>> {
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
    let requests = builder::build(details)?;

    // 5. Execute requests
    let results = executor::exec(requests).await?;

    Ok(results)
}
