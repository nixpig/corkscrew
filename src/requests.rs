use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, ops::Index, str::FromStr, time::Duration};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Requests {
    requests: Vec<RequestData>,
}

impl Default for Requests {
    fn default() -> Requests {
        Requests::new()
    }
}

impl Requests {
    fn new() -> Requests {
        Requests { requests: vec![] }
    }

    pub async fn exec(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let requests = self.requests.iter().filter(|r| {
            let has_resource = r.resource.is_some();
            let has_name = r.name.is_some();
            let run_all = config.request_names.is_empty();

            return has_resource
                && has_name
                && (run_all || config.request_names.contains(r.name.as_ref().unwrap()));
        });

        for request in requests {
            let mut url = String::from("");
            let mut headers = reqwest::header::HeaderMap::new();
            let mut params = HashMap::new();

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
                .json(body);

            let res = req.send().await?;
            println!("Name: {}", request.name.as_ref().unwrap());
            println!("Resource: {}", request.resource.as_ref().unwrap());
            println!("{:?} - {:?}", res.status(), res.url().to_string());

            let text = res.text().await?;

            let json: serde_json::Value = serde_json::from_str(&text).expect("should decode");
            println!("{:#?}", json);

            println!("-----")
        }

        Ok(())
    }

    fn parse(src: &Vec<RequestData>, target: &mut Requests, parent_index: usize) {
        for request_data in src {
            target.requests.push(RequestData::new());
            let pos = target.requests.len() - 1;

            if let Some(name) = &request_data.name {
                target.requests[pos].name = Some(name.clone());
            } else {
                panic!("All requests must have a name.");
            }

            target.requests[pos].host = match &request_data.host {
                Some(host) => Some(host.clone()),
                None => target.requests[parent_index].host.clone(),
            };

            target.requests[pos].scheme = match &request_data.scheme {
                Some(scheme) => Some(scheme.clone()),
                None => target.requests[parent_index].scheme.clone(),
            };

            target.requests[pos].port = match request_data.port {
                Some(port) => Some(port),
                None => target.requests[parent_index].port,
            };

            target.requests[pos].timeout = match request_data.timeout {
                Some(timeout) => Some(timeout),
                None => target.requests[parent_index].timeout,
            };

            target.requests[pos].resource = match &request_data.resource {
                Some(resource) => Some(resource.clone()),
                None => target.requests[parent_index].resource.clone(),
            };

            target.requests[pos].method = match &request_data.method {
                Some(method) => Some(method.clone()),
                None => target.requests[parent_index].method.clone(),
            };

            target.requests[pos].hash = match &request_data.hash {
                Some(hash) => Some(hash.clone()),
                None => target.requests[parent_index].hash.clone(),
            };

            target.requests[pos].params = match &request_data.params {
                Some(params) => Some(params.clone()),
                None => target.requests[parent_index].params.clone(),
            };

            target.requests[pos].headers = match &request_data.headers {
                Some(headers) => Some(headers.clone()),
                None => target.requests[parent_index].headers.clone(),
            };

            target.requests[pos].auth = match &request_data.auth {
                Some(auth) => Some(auth.clone()),
                None => target.requests[parent_index].auth.clone(),
            };

            target.requests[pos].body = match &request_data.body {
                Some(body) => Some(body.clone()),
                None => target.requests[parent_index].body.clone(),
            };

            if let Some(requests) = &request_data.requests {
                Requests::parse(requests, target, pos);
            }
        }
    }
}

impl FromStr for Requests {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Requests, Self::Err> {
        let yml: &Vec<RequestData> = &serde_yaml::from_str(s).unwrap();
        let mut flat_requests = Requests::new();
        Requests::parse(yml, &mut flat_requests, 0);

        Ok(flat_requests)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RequestData {
    pub requests: Option<Vec<RequestData>>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub scheme: Option<String>,
    pub port: Option<u16>,
    pub timeout: Option<u64>,
    pub resource: Option<String>,
    pub method: Option<String>,
    pub hash: Option<String>,
    pub params: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub auth: Option<AuthType>,
    pub body: Option<serde_json::Value>,
}

impl Default for RequestData {
    fn default() -> RequestData {
        RequestData::new()
    }
}

impl RequestData {
    pub fn new() -> RequestData {
        RequestData {
            requests: None,
            name: None,
            host: None,
            scheme: None,
            port: None,
            timeout: None,
            resource: None,
            method: None,
            hash: None,
            params: None,
            headers: None,
            auth: None,
            body: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Basic { username: String, password: String },
    Bearer { token: String },
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
mod test {
    use std::{collections::HashMap, error::Error, str::FromStr};

    use crate::requests::{AuthType, RequestData, Requests};

    #[test]
    fn test_parse_required_fields() -> Result<(), Box<dyn Error>> {
        let hosts_str_required_fields = "
- name: test_required_fields
  host: localhost
  resource: /api
";

        let got = Requests::from_str(hosts_str_required_fields)?;

        let want = vec![RequestData {
            name: Some(String::from("test_required_fields")),
            host: Some(String::from("localhost")),
            port: None,
            scheme: None,
            timeout: None,
            auth: None,
            body: None,
            hash: None,
            headers: None,
            method: None,
            params: None,
            resource: Some(String::from("/api")),
            requests: None,
        }];

        assert_eq!(got.requests, want);

        Ok(())
    }

    #[test]
    fn test_parse_nested_required_fields() -> Result<(), Box<dyn Error>> {
        let hosts_str_required_fields = "
- name: test_nested_required_fields
  host: localhost.1
  requests:
    - name: test_nested_required_fields_1
      resource: /api/1
    - name: test_nested_required_fields_2
      host: localhost.2
      resource: /api/2
      requests:
        - name: test_nested_required_fields_3
          host: localhost.3
          resource: /api/3
";

        let got = Requests::from_str(hosts_str_required_fields)?;

        let want = vec![
            RequestData {
                name: Some(String::from("test_nested_required_fields")),
                host: Some(String::from("localhost.1")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: None,
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_1")),
                host: Some(String::from("localhost.1")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/1")),
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_2")),
                host: Some(String::from("localhost.2")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/2")),
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_3")),
                host: Some(String::from("localhost.3")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/3")),
                requests: None,
            },
        ];

        assert_eq!(got.requests, want);

        Ok(())
    }

    #[test]
    fn test_parse_auth_basic() -> Result<(), Box<dyn Error>> {
        let requests_str_basic_auth = "
- name: test_basic_auth
  host: localhost
  auth: !basic
    username: test_username
    password: test_password
";

        let got = Requests::from_str(requests_str_basic_auth)?;

        let want = vec![RequestData {
            name: Some(String::from("test_basic_auth")),
            host: Some(String::from("localhost")),
            port: None,
            timeout: None,
            scheme: None,
            auth: Some(AuthType::Basic {
                username: String::from("test_username"),
                password: String::from("test_password"),
            }),
            body: None,
            hash: None,
            headers: None,
            method: None,
            params: None,
            resource: None,
            requests: None,
        }];

        assert_eq!(got.requests, want);

        Ok(())
    }

    #[test]
    fn test_parse_bearer_auth() -> Result<(), Box<dyn Error>> {
        let requests_str_bearer_auth = "
- name: test_bearer_auth
  host: localhost
  auth: !bearer
    token: 50m3.b34r3r.t0k?n
";

        let got = Requests::from_str(requests_str_bearer_auth)?;

        let want = vec![RequestData {
            name: Some(String::from("test_bearer_auth")),
            host: Some(String::from("localhost")),
            port: None,
            timeout: None,
            scheme: None,
            auth: Some(AuthType::Bearer {
                token: String::from("50m3.b34r3r.t0k?n"),
            }),
            body: None,
            hash: None,
            headers: None,
            method: None,
            params: None,
            resource: None,
            requests: None,
        }];

        assert_eq!(got.requests, want);

        Ok(())
    }

    #[test]
    fn test_parse_all_unconstrained_fields() -> Result<(), Box<dyn Error>> {
        let requests_str_all_unconstrained_fields = "
    - name: test_unconstrained_fields
      host: localhost
      timeout: 15
      port: 3000
      scheme: https
      requests:
        - name: test_post_1
          method: post
          resource: /api/test_1
          hash: hash_location_1
          headers:
            Accept-Language: en-US,en;q=0.8
            User-Agent: Mozilla/5.0 Firefox/50.0
          params:
            param1: value1
            param2: value2
          body:
            prop1a: val1a
            prop1b:
              prop2a: val2a
              prop2b:
                prop3a: val3a
                prop3b: val3b
        - name: test_post_2
          method: patch
          resource: /api/test_2
          hash: hash_location_2
          params:
            paramA: valueA
            paramB: valueB

    ";

        let got = Requests::from_str(requests_str_all_unconstrained_fields)?;

        let want = vec![
            RequestData {
                name: Some(String::from("test_unconstrained_fields")),
                host: Some(String::from("localhost")),
                timeout: Some(15),
                port: Some(3000),
                scheme: Some(String::from("https")),
                body: None,
                auth: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: None,
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_post_1")),
                method: Some(String::from("post")),
                resource: Some(String::from("/api/test_1")),
                hash: Some(String::from("hash_location_1")),
                host: Some(String::from("localhost")),
                port: Some(3000),
                timeout: Some(15),
                scheme: Some(String::from("https")),
                requests: None,
                auth: None,
                headers: Some(HashMap::from([
                    (
                        String::from("Accept-Language"),
                        String::from("en-US,en;q=0.8"),
                    ),
                    (
                        String::from("User-Agent"),
                        String::from("Mozilla/5.0 Firefox/50.0"),
                    ),
                ])),
                params: Some(HashMap::from([
                    (String::from("param1"), String::from("value1")),
                    (String::from("param2"), String::from("value2")),
                ])),
                body: Some(
                    serde_json::from_str(
                        "{
                                \"prop1a\": \"val1a\",
                                \"prop1b\": {
                                    \"prop2a\": \"val2a\",
                                    \"prop2b\": {
                                        \"prop3a\": \"val3a\",
                                        \"prop3b\": \"val3b\"
                                    }
                                }
                            }",
                    )
                    .unwrap(),
                ),
            },
            RequestData {
                name: Some(String::from("test_post_2")),
                method: Some(String::from("patch")),
                resource: Some(String::from("/api/test_2")),
                hash: Some(String::from("hash_location_2")),
                host: Some(String::from("localhost")),
                port: Some(3000),
                timeout: Some(15),
                scheme: Some(String::from("https")),
                requests: None,
                auth: None,
                body: None,
                headers: None,
                params: Some(HashMap::from([
                    (String::from("paramA"), String::from("valueA")),
                    (String::from("paramB"), String::from("valueB")),
                ])),
            },
        ];

        assert_eq!(got.requests, want);

        Ok(())
    }
}
