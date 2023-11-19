use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Method {
    fn default() -> Self {
        Method::Get
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Timeout(u32);

impl Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Hosts(Vec<Host>);

#[derive(Debug)]
pub struct HostsParseError(Box<dyn Error>);
impl<E: Error + 'static> From<E> for HostsParseError {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}

impl FromStr for Hosts {
    type Err = HostsParseError;

    fn from_str(s: &str) -> Result<Self, HostsParseError> {
        let hosts: Self = serde_yaml::from_str(s)?;
        Ok(hosts)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Host {
    name: String,
    host: String,
    port: Option<u16>,
    #[serde(default = "Timeout::default")]
    timeout: Timeout,
    requests: Vec<Request>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Request {
    name: String,
    resource: Option<String>,
    #[serde(default = "Method::default")]
    method: Method,
    hash: Option<String>,
    params: Option<HashMap<String, String>>,
    headers: Option<HashMap<String, String>>,
    auth: Option<AuthType>,
    body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum AuthType {
    Basic { username: String, password: String },
    Bearer { token: String },
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::str::FromStr;

    use super::{AuthType, Host, Hosts, HostsParseError, Method, Request, Timeout};

    #[test]
    fn test_parse_required_fields() -> Result<(), HostsParseError> {
        let hosts_str_required_fields = "
- name: test_required_fields
  host: localhost
  requests: 
    - name: test_request_name
";

        let hosts = Hosts::from_str(hosts_str_required_fields)?;

        let expected = Hosts(vec![Host {
            name: String::from("test_required_fields"),
            host: String::from("localhost"),
            port: None,
            timeout: Timeout(30),
            requests: vec![Request {
                auth: None,
                body: None,
                hash: None,
                headers: None,
                method: Method::Get,
                params: None,
                resource: None,
                name: String::from("test_request_name"),
            }],
        }]);

        assert_eq!(hosts, expected);

        Ok(())
    }

    #[test]
    fn test_parse_enum_fields() -> Result<(), HostsParseError> {
        let hosts_str_enum_fields = "
- name: test_enum_fields
  host: localhost
  timeout: 10
  requests: 
    - name: test_post
      method: !post
";

        let hosts = Hosts::from_str(hosts_str_enum_fields)?;

        let expected = Hosts(vec![Host {
            name: String::from("test_enum_fields"),
            host: String::from("localhost"),
            timeout: Timeout(10),
            port: None,
            requests: vec![Request {
                name: String::from("test_post"),
                method: Method::Post,
                auth: None,
                body: None,
                hash: None,
                headers: None,
                params: None,
                resource: None,
            }],
        }]);

        assert_eq!(hosts, expected);
        Ok(())
    }

    #[test]
    fn test_parse_all_unconstrained_fields() -> Result<(), HostsParseError> {
        let hosts_str_all_unconstrained_fields = "
- name: test_unconstrained_fields
  host: localhost
  timeout: 15
  port: 3000
  requests: 
    - name: test_post
      method: !post
      resource: /api/test
      hash: hash_location
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
        
";

        let hosts = Hosts::from_str(hosts_str_all_unconstrained_fields)?;

        let expected = Hosts(vec![Host {
            name: String::from("test_unconstrained_fields"),
            host: String::from("localhost"),
            timeout: Timeout(15),
            port: Some(3000),
            requests: vec![Request {
                name: String::from("test_post"),
                method: Method::Post,
                resource: Some(String::from("/api/test")),
                hash: Some(String::from("hash_location")),
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
                auth: None,
            }],
        }]);

        assert_eq!(hosts, expected);

        Ok(())
    }

    #[test]
    fn test_parse_auth_basic() -> Result<(), HostsParseError> {
        let hosts_str_basic_auth = "
- name: test_basic_auth
  host: localhost
  requests:
    - name: basic_auth_request
      auth: !basic 
        username: test_username
        password: test_password
";

        let hosts = Hosts::from_str(hosts_str_basic_auth)?;

        let expected = Hosts(vec![Host {
            name: String::from("test_basic_auth"),
            host: String::from("localhost"),
            port: None,
            timeout: Timeout(30),
            requests: vec![Request {
                name: String::from("basic_auth_request"),
                auth: Some(AuthType::Basic {
                    username: String::from("test_username"),
                    password: String::from("test_password"),
                }),
                body: None,
                hash: None,
                headers: None,
                method: Method::Get,
                params: None,
                resource: None,
            }],
        }]);

        assert_eq!(hosts, expected);

        Ok(())
    }

    #[test]
    fn test_parse_auth_bearer() -> Result<(), HostsParseError> {
        let hosts_str_bearer_auth = "
- name: test_bearer_auth
  host: localhost
  requests:
    - name: bearer_auth_request
      auth: !bearer 
        token: abcd$1234.231&4dfs-asdfjsdv.vsd
";

        let hosts = Hosts::from_str(hosts_str_bearer_auth)?;

        let expected = Hosts(vec![Host {
            name: String::from("test_bearer_auth"),
            host: String::from("localhost"),
            port: None,
            timeout: Timeout(30),
            requests: vec![Request {
                name: String::from("bearer_auth_request"),
                auth: Some(AuthType::Bearer {
                    token: String::from("abcd$1234.231&4dfs-asdfjsdv.vsd"),
                }),
                body: None,
                hash: None,
                headers: None,
                method: Method::Get,
                params: None,
                resource: None,
            }],
        }]);

        assert_eq!(hosts, expected);

        Ok(())
    }

    #[test]
    fn test_parse_multiple_hosts() -> Result<(), HostsParseError> {
        let hosts_str_multiple = "
- name: test_host_1
  host: foo.localhost
  requests: 
    - name: test_host_1_post_1
    - name: test_host_1_post_2
- name: test_host_2
  host: bar.localhost
  requests: 
    - name: test_host_2_post_1
    - name: test_host_2_post_2
";

        let hosts = Hosts::from_str(hosts_str_multiple)?;

        let expected = Hosts(vec![
            Host {
                name: String::from("test_host_1"),
                host: String::from("foo.localhost"),
                timeout: Timeout(30),
                port: None,
                requests: vec![
                    Request {
                        name: String::from("test_host_1_post_1"),
                        method: Method::Get,
                        auth: None,
                        body: None,
                        hash: None,
                        headers: None,
                        params: None,
                        resource: None,
                    },
                    Request {
                        name: String::from("test_host_1_post_2"),
                        method: Method::Get,
                        auth: None,
                        body: None,
                        hash: None,
                        headers: None,
                        params: None,
                        resource: None,
                    },
                ],
            },
            Host {
                name: String::from("test_host_2"),
                host: String::from("bar.localhost"),
                timeout: Timeout(30),
                port: None,
                requests: vec![
                    Request {
                        name: String::from("test_host_2_post_1"),
                        method: Method::Get,
                        auth: None,
                        body: None,
                        hash: None,
                        headers: None,
                        params: None,
                        resource: None,
                    },
                    Request {
                        name: String::from("test_host_2_post_2"),
                        method: Method::Get,
                        auth: None,
                        body: None,
                        hash: None,
                        headers: None,
                        params: None,
                        resource: None,
                    },
                ],
            },
        ]);

        assert_eq!(hosts, expected);
        Ok(())
    }
}
