use cli_table;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Index, time::Duration};

use crate::{
    error::{handle_error, CorkscrewError},
    settings::Settings,
    types::{AuthType, RequestData},
};

#[derive(cli_table::Table)]
pub struct Output {
    #[table(title = "#")]
    num: usize,
    #[table(title = "Name")]
    name: String,
    #[table(title = "Method")]
    method: String,
    #[table(title = "Resource")]
    resource: String,
    #[table(title = "Status")]
    status: String,
    // #[table(data = "Data")]
    // data: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Requests {
    pub requests: Vec<RequestData>,
}

impl Default for Requests {
    fn default() -> Requests {
        Requests::new()
    }
}

impl Requests {
    pub fn new() -> Requests {
        Requests { requests: vec![] }
    }

    pub async fn exec(config: &Requests) -> Result<Vec<Output>, CorkscrewError> {
        let mut output: Vec<Output> = vec![];

        for (num, request) in config.requests.iter().enumerate() {
            let mut url = String::from("");
            let mut headers = reqwest::header::HeaderMap::new();
            let mut params = HashMap::new();

            let t = HeaderValue::from_str("application/x-www-form-urlencoded")
                .expect("to parse header");
            headers.append("Content-Type", t);

            match &request.scheme {
                Some(scheme) => url.push_str(scheme),
                None => url.push_str("http"),
            }

            url.push_str("://");

            if let Some(auth) = &request.auth {
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

            match &request.host {
                Some(host) => url.push_str(host),
                None => panic!("Host is required"),
            }

            if let Some(port) = &request.port {
                url.push_str(&format!(":{port}"));
            }

            if let Some(resource) = &request.resource {
                url.push_str(resource);
            }

            if let Some(hash) = &request.hash {
                url.push_str(&format!("#{hash}"));
            }

            if let Some(h) = &request.headers {
                headers = h.try_into().expect("Expected to receive valid headers.")
            }

            let body = &request.body.clone().unwrap_or(serde_json::Value::Null);
            let form = &request.form.clone().unwrap_or(HashMap::new());

            let method = match &request.method {
                Some(m) => Method {}[m].clone(),
                None => Method {}["get"].clone(),
            };

            if let Some(p) = &request.params {
                for (name, value) in p.iter() {
                    params.insert(name, value);
                }
            }

            let timeout = request.timeout.unwrap_or(10);

            let req = reqwest::Client::new()
                .request(method, &url)
                .timeout(Duration::from_secs(timeout))
                .headers(headers)
                .query(&params)
                .form(form)
                .json(body);

            if let Ok(res) = req.send().await {
                let default_method = String::from("get");
                let name = request.name.as_ref().unwrap();
                let status = res.status();
                let method = request.method.as_ref().unwrap_or(&default_method);
                let _url = res.url().to_string();

                let resource = request.resource.as_ref().unwrap();

                if let Ok(text) = res.text().await {
                    let json: serde_json::Value =
                        serde_json::from_str(&text).expect("should decode");

                    output.push(Output {
                        num,
                        resource: String::from(resource),
                        name: String::from(name),
                        status: status.to_string(),
                        method: method.to_string(),
                        // data: json.to_string(),
                    });
                } else {
                    handle_error(CorkscrewError::Request)
                }
            } else {
                handle_error(CorkscrewError::Request)
            }
        }

        Ok(output)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Method {}
impl Index<&'_ str> for Method {
    type Output = reqwest::Method;

    fn index(&self, index: &str) -> &reqwest::Method {
        match index {
            "get" => &reqwest::Method::GET,
            "post" => &reqwest::Method::POST,
            "put" => &reqwest::Method::PUT,
            "patch" => &reqwest::Method::PATCH,
            "delete" => &reqwest::Method::DELETE,
            _ => &reqwest::Method::GET,
        }
    }
}

#[cfg(test)]
mod test {}
