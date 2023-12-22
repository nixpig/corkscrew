use serde::{Deserialize, Serialize};

use crate::types::auth::AuthType;
use std::collections::HashMap;

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
    pub content: Option<String>,
    pub body: Option<serde_json::Value>,
    pub form: Option<HashMap<String, String>>,
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
            content: None,
            body: None,
            form: None,
        }
    }
}
