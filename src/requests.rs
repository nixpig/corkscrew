use serde::{Deserialize, Serialize};

use crate::exec::{Exec, ExecError};
use crate::load::{Load, LoadError};
use crate::parse::{Parse, ParseError};
use crate::request::{AuthType, RequestData, RequestExec, UnwrappedRequestData};
use std::fs;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Requests {
    config: Config,
    requests_str: String,
    requests_data: Vec<RequestData>,
}

impl Load for Requests {
    fn load(&mut self) -> Result<&mut Self, LoadError> {
        self.requests_str = fs::read_to_string(&self.config.config_path)?;

        Ok(self)
    }
}

impl Parse for Requests {
    fn parse(&mut self) -> Result<&mut Self, ParseError> {
        let requests_data: Vec<RequestData> = serde_yaml::from_str(&self.requests_str)?;

        // Flatten the requests
        fn r(requests_data: &Vec<RequestData>, o: &mut Requests) {
            for request_data in requests_data {
                if let Some(requests) = &request_data.requests {
                    r(requests, o);
                }
                // add props to capture
                o.requests_data.push(RequestData::new());

                if let Some(name) = request_data.name.clone() {
                    let len = o.requests_data.len();
                    o.requests_data[len - 1].name = Some(name);
                }
            }
        }

        r(&requests_data, self);
        println!("PARSED:");
        println!("{:#?}", self.requests_data);

        Ok(self)
    }
}

impl Exec for Requests {
    fn exec(&self) -> Result<&Self, ExecError> {
        for request in &self.requests_data {
            // exec
            todo!();
        }

        Ok(self)
    }
}

impl Requests {
    pub fn new(config: Config) -> Requests {
        Requests {
            config,
            requests_str: String::from(""),
            requests_data: vec![],
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
