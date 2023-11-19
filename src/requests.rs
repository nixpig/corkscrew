use crate::{
    config::Config,
    hosts::{self, Hosts},
};
use std::error::Error;

#[derive(Debug)]
pub struct Requests {
    data: Vec<reqwest::RequestBuilder>,
}

impl Requests {
    pub fn from_hosts(hosts: Hosts, config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut requests = vec![];

        hosts.data.iter().for_each(|host| {
            host.requests
                .iter()
                .filter(|request| {
                    config.request_names.len() != 0 && !config.request_names.contains(&request.name)
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
                        url.push_str(":");
                        url.push_str(&port.to_string());
                    }

                    if let Some(resource) = &request.resource {
                        url.push_str(resource);
                    }

                    if let Some(hash) = &request.hash {
                        url.push_str("#");
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

                    let mut body = &serde_json::Value::Null;

                    if let Some(b) = &request.body {
                        body = b
                    }

                    let request = reqwest::Client::new()
                        .request(method, &url)
                        .headers(headers)
                        .json(body);

                    requests.push(request);
                })
        });

        Ok(Requests { data: requests })
    }

    // execute synchronously
    pub fn exec(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    // execute asynchronously
    pub async fn exec_async(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
