use reqwest::header::HeaderValue;

use crate::{
    config::Config,
    hosts::{self, Hosts, AuthType},
};
use std::error::Error;

#[derive(Debug)]
pub struct Requests {}

impl Requests {
    pub async fn send_from_hosts(hosts: Hosts, config: &Config) -> Result<(), Box<dyn Error>> {
        let mut requests = vec![];

        hosts.data.iter().for_each(|host| {
            host.requests
                .iter()
                .filter(|request| {
                    config.request_names.is_empty() || config.request_names.contains(&request.name)
                })
                .for_each(|request| {
                    let mut url = String::new();

                    let method = match request.method {
                        hosts::Method::Get => reqwest::Method::GET,
                        hosts::Method::Post => reqwest::Method::POST,
                        hosts::Method::Put => reqwest::Method::PUT,
                        hosts::Method::Patch => reqwest::Method::PATCH,
                        hosts::Method::Delete => reqwest::Method::DELETE,
                    };

                    let scheme: String = match &host.scheme {
                        Some(s) => String::from(s),
                        None => String::from("http"),
                    };

                    url.push_str(&scheme);
                    url.push_str("://");
                    url.push_str(&host.host);
                    if let Some(port) = &host.port {
                        url.push_str(&String::from(":"));
                        url.push_str(&port.to_string());
                    }

                    if let Some(resource) = &request.resource {
                        url.push_str(resource);
                    }

                    if let Some(hash) = &request.hash {
                        url.push_str(&String::from("#"));
                        url.push_str(hash);
                    }

                    if let Some(params) = &request.params {
                        let p: Vec<String> =
                            params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();

                        let mut query = String::from("?");
                        query.push_str(p.join("&").as_str());
                    }

                    let mut headers = reqwest::header::HeaderMap::new();

                    if let Some(h) = &request.headers {
                        headers = h.try_into().expect("Expected to receive valid headers.");
                    }


                    if let Some(auth) = &request.auth {
                        match auth {
                            AuthType::Bearer { token } => {
                                let bearer_token_header_value = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
                                headers.insert("Authorization", bearer_token_header_value);
                            },
                            AuthType::Basic { username, password } => {
                                todo!("implement handling of basic auth");
                            },
                        };
                    };

                    let mut body = &serde_json::Value::Null;

                    if let Some(b) = &request.body {
                        body = b
                    }

                    let req = reqwest::Client::new()
                        .request(method, &url)
                        .headers(headers)
                        .json(body);

                    requests.push((&request.name, req));
                });
        });

        for (name, req) in requests {
            let res = req.send().await?;
            let status = res.status();

            println!("{} | {}", name, status);

            match status.as_u16() {
                200..=299 => {
                    let body = res.text().await?;
                    println!("{:#}\n", body);
                }
                400..=599 => {
                    let error = res.text().await?;
                    println!("Error: {}", error);
                }
                _ => {
                    println!("TODO: handle other status codes ({})", status);
                }
            }
        }

        Ok(())
    }
}
