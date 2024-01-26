use std::{collections::HashMap, error::Error};

pub async fn exec(
    requests: HashMap<String, reqwest::RequestBuilder>,
) -> Result<HashMap<String, reqwest::Response>, Box<dyn Error>> {
    let mut responses = HashMap::<String, reqwest::Response>::new();

    for (request_name, request) in requests {
        match request.send().await {
            Ok(response) => {
                responses.insert(request_name, response);
            }
            Err(e) => eprintln!("{}", e),
        };
    }

    Ok(responses)
}
