use crate::exec::{Exec, ExecError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RequestExec {
    pub url: String,
    pub method: reqwest::Method,
    pub headers: reqwest::header::HeaderMap,
    pub body: serde_json::Value,
    pub name: String,
}

impl Exec for RequestExec {
    fn exec(&self) -> Result<&Self, ExecError> {
        println!(
            "Executing... 
        Name: {}
        URL: {}
        Method: {:?}
        Headers: {:#?}
        Body: {:#?}",
            &self.name, &self.url, &self.method, &self.headers, &self.body
        );

        // let req = reqwest::Client::new()
        //     .request(self.method, &self.url)
        //     .headers(self.headers)
        //     .json(&self.body);

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
pub struct UnwrappedRequestData {
    pub requests: Vec<RequestData>,
    pub name: String,
    pub host: String,
    pub scheme: String,
    pub port: u16,
    pub timeout: u16,
    pub resource: String,
    pub method: Method,
    pub hash: String,
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub auth: AuthType,
    pub body: serde_json::Value,
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
