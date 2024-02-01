use std::{collections::HashMap, error::Error};

pub async fn exec(
    requests: HashMap<String, reqwest::Request>,
) -> Result<HashMap<String, reqwest::Response>, Box<dyn Error>> {
    let mut responses = HashMap::<String, reqwest::Response>::new();

    for (request_name, request) in requests {
        match reqwest::Client::new().execute(request).await {
            Ok(response) => {
                responses.insert(request_name, response);
            }
            Err(e) => eprintln!("{}", e),
        };
    }

    Ok(responses)
}
