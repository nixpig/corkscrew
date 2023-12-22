use serde::{Deserialize, Serialize};

use crate::types::Detail;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Requests {
    pub requests: Vec<Detail>,
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
}
