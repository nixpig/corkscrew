use serde::{Deserialize, Serialize};

use crate::types::auth::AuthType;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Detail {
    pub requests: Option<Vec<Detail>>,
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
    pub content: Option<String>,
    pub body: Option<serde_json::Value>,
    pub form: Option<HashMap<String, String>>,
}

impl Default for Detail {
    fn default() -> Detail {
        Detail::new()
    }
}

impl Detail {
    pub fn new() -> Detail {
        Detail {
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
            content: None,
            body: None,
            form: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Detail;
    use std::error::Error;

    #[test]
    fn test_default_detail_construction() -> Result<(), Box<dyn Error>> {
        let got = Detail::default();

        let expected = Detail {
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
            content: None,
            body: None,
            form: None,
        };

        assert_eq!(
            got, expected,
            "should construct Detail struct with None for all fields"
        );

        Ok(())
    }

    #[test]
    fn test_new_detail_construction() -> Result<(), Box<dyn Error>> {
        let got = Detail::new();

        let expected = Detail {
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
            content: None,
            body: None,
            form: None,
        };

        assert_eq!(
            got, expected,
            "should construct Detail struct with None for all fields"
        );

        Ok(())
    }
}
