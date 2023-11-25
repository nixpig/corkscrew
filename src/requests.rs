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

            let mut body = &serde_json::Value::Null;
            if let Some(b) = &request.body {
                body = b;
            }

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

            // o.requests[pos].auth = match request_data.auth {
            //     Some(auth) => Some(auth),
            //     None => o.requests[parent_index].auth,
            // };

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;
//     use std::str::FromStr;
//
//     use super::{AuthType, Host, Hosts, RequestParseError, Method, Request, Timeout};
//
//     #[test]
//     fn test_parse_required_fields() -> Result<(), RequestParseError> {
//         let hosts_str_required_fields = "
// - name: test_required_fields
//   host: localhost
//   requests:
//     - name: test_request_name
// ";
//
//         let hosts = Hosts::from_str(hosts_str_required_fields)?;
//
//         let expected = Hosts::new(vec![Host {
//             name: String::from("test_required_fields"),
//             host: String::from("localhost"),
//             port: None,
//             scheme: None,
//             timeout: Timeout(30),
//             requests: vec![Request {
//                 auth: None,
//                 body: None,
//                 hash: None,
//                 headers: None,
//                 method: Method::Get,
//                 params: None,
//                 resource: None,
//                 name: String::from("test_request_name"),
//             }],
//         }]);
//
//         assert_eq!(hosts, expected);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_parse_enum_fields() -> Result<(), RequestParseError> {
//         let hosts_str_enum_fields = "
// - name: test_enum_fields
//   host: localhost
//   timeout: 10
//   requests:
//     - name: test_post
//       method: !post
// ";
//
//         let hosts = Hosts::from_str(hosts_str_enum_fields)?;
//
//         let expected = Hosts::new(vec![Host {
//             name: String::from("test_enum_fields"),
//             host: String::from("localhost"),
//             scheme: None,
//             timeout: Timeout(10),
//             port: None,
//             requests: vec![Request {
//                 name: String::from("test_post"),
//                 method: Method::Post,
//                 auth: None,
//                 body: None,
//                 hash: None,
//                 headers: None,
//                 params: None,
//                 resource: None,
//             }],
//         }]);
//
//         assert_eq!(hosts, expected);
//         Ok(())
//     }
//
//     #[test]
//     fn test_parse_all_unconstrained_fields() -> Result<(), RequestParseError> {
//         let hosts_str_all_unconstrained_fields = "
// - name: test_unconstrained_fields
//   host: localhost
//   timeout: 15
//   port: 3000
//   scheme: https
//   requests:
//     - name: test_post
//       method: !post
//       resource: /api/test
//       hash: hash_location
//       headers:
//         Accept-Language: en-US,en;q=0.8
//         User-Agent: Mozilla/5.0 Firefox/50.0
//       params:
//         param1: value1
//         param2: value2
//       body:
//         prop1a: val1a
//         prop1b:
//           prop2a: val2a
//           prop2b:
//             prop3a: val3a
//             prop3b: val3b
//
// ";
//
//         let hosts = Hosts::from_str(hosts_str_all_unconstrained_fields)?;
//
//         let expected = Hosts::new(vec![Host {
//             name: String::from("test_unconstrained_fields"),
//             host: String::from("localhost"),
//             timeout: Timeout(15),
//             port: Some(3000),
//             scheme: Some(String::from("https")),
//             requests: vec![Request {
//                 name: String::from("test_post"),
//                 method: Method::Post,
//                 resource: Some(String::from("/api/test")),
//                 hash: Some(String::from("hash_location")),
//                 headers: Some(HashMap::from([
//                     (
//                         String::from("Accept-Language"),
//                         String::from("en-US,en;q=0.8"),
//                     ),
//                     (
//                         String::from("User-Agent"),
//                         String::from("Mozilla/5.0 Firefox/50.0"),
//                     ),
//                 ])),
//                 params: Some(HashMap::from([
//                     (String::from("param1"), String::from("value1")),
//                     (String::from("param2"), String::from("value2")),
//                 ])),
//                 body: Some(
//                     serde_json::from_str(
//                         "{
//                             \"prop1a\": \"val1a\",
//                             \"prop1b\": {
//                                 \"prop2a\": \"val2a\",
//                                 \"prop2b\": {
//                                     \"prop3a\": \"val3a\",
//                                     \"prop3b\": \"val3b\"
//                                 }
//                             }
//                         }",
//                     )
//                     .unwrap(),
//                 ),
//                 auth: None,
//             }],
//         }]);
//
//         assert_eq!(hosts, expected);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_parse_auth_basic() -> Result<(), RequestParseError> {
//         let hosts_str_basic_auth = "
// - name: test_basic_auth
//   host: localhost
//   scheme: http
//   requests:
//     - name: basic_auth_request
//       auth: !basic
//         username: test_username
//         password: test_password
// ";
//
//         let hosts = Hosts::from_str(hosts_str_basic_auth)?;
//
//         let expected = Hosts::new(vec![Host {
//             name: String::from("test_basic_auth"),
//             host: String::from("localhost"),
//             port: None,
//             timeout: Timeout(30),
//             scheme: Some(String::from("http")),
//             requests: vec![Request {
//                 name: String::from("basic_auth_request"),
//                 auth: Some(AuthType::Basic {
//                     username: String::from("test_username"),
//                     password: String::from("test_password"),
//                 }),
//                 body: None,
//                 hash: None,
//                 headers: None,
//                 method: Method::Get,
//                 params: None,
//                 resource: None,
//             }],
//         }]);
//
//         assert_eq!(hosts, expected);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_parse_auth_bearer() -> Result<(), RequestParseError> {
//         let hosts_str_bearer_auth = "
// - name: test_bearer_auth
//   host: localhost
//   requests:
//     - name: bearer_auth_request
//       auth: !bearer
//         token: abcd$1234.231&4dfs-asdfjsdv.vsd
// ";
//
//         let hosts = Hosts::from_str(hosts_str_bearer_auth)?;
//
//         let expected = Hosts::new(vec![Host {
//             name: String::from("test_bearer_auth"),
//             host: String::from("localhost"),
//             port: None,
//             scheme: None,
//             timeout: Timeout(30),
//             requests: vec![Request {
//                 name: String::from("bearer_auth_request"),
//                 auth: Some(AuthType::Bearer {
//                     token: String::from("abcd$1234.231&4dfs-asdfjsdv.vsd"),
//                 }),
//                 body: None,
//                 hash: None,
//                 headers: None,
//                 method: Method::Get,
//                 params: None,
//                 resource: None,
//             }],
//         }]);
//
//         assert_eq!(hosts, expected);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_parse_multiple_hosts() -> Result<(), RequestParseError> {
//         let hosts_str_multiple = "
// - name: test_host_1
//   host: foo.localhost
//   requests:
//     - name: test_host_1_post_1
//     - name: test_host_1_post_2
// - name: test_host_2
//   host: bar.localhost
//   requests:
//     - name: test_host_2_post_1
//     - name: test_host_2_post_2
// ";
//
//         let hosts = Hosts::from_str(hosts_str_multiple)?;
//
//         let expected = Hosts::new(vec![
//             Host {
//                 name: String::from("test_host_1"),
//                 host: String::from("foo.localhost"),
//                 timeout: Timeout(30),
//                 port: None,
//                 scheme: None,
//                 requests: vec![
//                     Request {
//                         name: String::from("test_host_1_post_1"),
//                         method: Method::Get,
//                         auth: None,
//                         body: None,
//                         hash: None,
//                         headers: None,
//                         params: None,
//                         resource: None,
//                     },
//                     Request {
//                         name: String::from("test_host_1_post_2"),
//                         method: Method::Get,
//                         auth: None,
//                         body: None,
//                         hash: None,
//                         headers: None,
//                         params: None,
//                         resource: None,
//                     },
//                 ],
//             },
//             Host {
//                 name: String::from("test_host_2"),
//                 host: String::from("bar.localhost"),
//                 timeout: Timeout(30),
//                 scheme: None,
//                 port: None,
//                 requests: vec![
//                     Request {
//                         name: String::from("test_host_2_post_1"),
//                         method: Method::Get,
//                         auth: None,
//                         body: None,
//                         hash: None,
//                         headers: None,
//                         params: None,
//                         resource: None,
//                     },
//                     Request {
//                         name: String::from("test_host_2_post_2"),
//                         method: Method::Get,
//                         auth: None,
//                         body: None,
//                         hash: None,
//                         headers: None,
//                         params: None,
//                         resource: None,
//                     },
//                 ],
//             },
//         ]);
//
//         assert_eq!(hosts, expected);
//         Ok(())
//     }
// }
