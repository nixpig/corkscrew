use crate::exec::{Exec, ExecError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Request {}

impl Exec for Request {
    fn exec(&self) -> Result<&Self, ExecError> {
        todo!();
        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RequestData {
    pub requests: Option<Vec<RequestData>>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub scheme: Option<String>,
    pub port: Option<u16>,
    pub timeout: Option<u16>,
    pub resource: Option<String>,
    pub method: Option<Method>,
    pub hash: Option<String>,
    pub params: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub auth: Option<AuthType>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Basic { username: String, password: String },
    Bearer { token: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}
