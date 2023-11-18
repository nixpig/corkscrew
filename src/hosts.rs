use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

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
    requests: Vec<Request>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Request {
    name: String,
    path: Option<String>,
    method: Option<String>,
    hash: Option<String>,
    params: Option<HashMap<String, String>>,
    headers: Option<HashMap<String, String>>,
    auth: Option<Auth>,
    body: Option<Body>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Body {
    // type: Option<String>, // optional, defaults to JSON
    // TODO: need to add constraints around this, e.g. if type is JSON
    // TODO: need to automatically work out if content is string or json/hashmap if content is
    // string or serde_json::Value/hashmap
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Auth {
    // TODO: constraints around this, e.g. if type is bearer then name of next prop is token, if
    // type is basic then name of next props are username, password
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{Host, Hosts, HostsParseError, Request};

    #[test]
    fn test_parse_required_fields() -> Result<(), HostsParseError> {
        let hosts_str_missing_fields = "
- name: test_host_name
  host: localhost
  requests: 
    - name: test_request_name
";

        let hosts = Hosts::from_str(hosts_str_missing_fields)?;

        println!("HOSTS IN HERE: {:?}", hosts);

        let expected = Hosts(vec![Host {
            name: String::from("test_host_name"),
            host: String::from("localhost"),
            port: None,
            requests: vec![Request {
                auth: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                path: None,
                name: String::from("test_request_name"),
            }],
        }]);

        assert_eq!(hosts, expected);

        Ok(())
    }
}
