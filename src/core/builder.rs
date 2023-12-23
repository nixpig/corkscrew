use reqwest::header::HeaderValue;

use crate::types::{AuthType, Detail, Method};
use std::{collections::HashMap, error::Error, time::Duration};

pub async fn build(details: Vec<Detail>) -> Result<Vec<reqwest::RequestBuilder>, Box<dyn Error>> {
    let mut requests: Vec<reqwest::RequestBuilder> = vec![];

    for request_detail in details.iter() {
        let mut url = String::from("");
        let mut headers = reqwest::header::HeaderMap::new();
        let mut params = HashMap::new();

        match &request_detail.scheme {
            Some(scheme) => url.push_str(scheme),
            None => url.push_str("http"),
        }

        url.push_str("://");

        if let Some(auth) = &request_detail.auth {
            match auth {
                AuthType::Bearer { token } => {
                    let bearer_token_header_value =
                        HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
                    headers.append("Authorization", bearer_token_header_value);
                }
                AuthType::Basic { username, password } => {
                    url.push_str(&format!("{username}:{password}"));
                }
            }
        }

        match &request_detail.host {
            Some(host) => url.push_str(host),
            None => panic!("Host is required"),
        }

        if let Some(port) = &request_detail.port {
            url.push_str(&format!(":{port}"));
        }

        if let Some(resource) = &request_detail.resource {
            url.push_str(resource);
        }

        if let Some(hash) = &request_detail.hash {
            url.push_str(&format!("#{hash}"));
        }

        if let Some(h) = &request_detail.headers {
            headers = h.try_into().expect("Expected to receive valid headers.")
        }

        let body = request_detail
            .body
            .clone()
            .unwrap_or(serde_json::Value::Null);

        let form = request_detail.form.clone().unwrap_or(HashMap::new());

        let method = match &request_detail.method {
            Some(m) => Method {}[m].clone(),
            None => Method {}["get"].clone(),
        };

        if let Some(p) = &request_detail.params {
            for (name, value) in p.iter() {
                params.insert(name, value);
            }
        }

        let timeout = request_detail.timeout.unwrap_or(10);

        if let Some(content) = &request_detail.content {
            match content.as_str() {
                "form" => {
                    let req = reqwest::Client::new()
                        .request(method, &url)
                        .timeout(Duration::from_secs(timeout))
                        .headers(headers)
                        .query(&params)
                        .form(&form);

                    requests.push(req);
                }
                _ => {
                    let req = reqwest::Client::new()
                        .request(method, &url)
                        .timeout(Duration::from_secs(timeout))
                        .headers(headers)
                        .query(&params)
                        .json(&body);

                    requests.push(req);
                }
            }
        } else {
            let req = reqwest::Client::new()
                .request(method, &url)
                .timeout(Duration::from_secs(timeout))
                .headers(headers)
                .query(&params)
                .json(&body);

            requests.push(req);
        }
    }

    Ok(requests)
}

#[cfg(test)]

mod test {
    use crate::builder::build;
    use reqwest::Client;
    use std::error::Error;

    async fn test_build_requests() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
