use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Body {
    pub(crate)  content: String,
    pub(crate)  encoding: String,
    #[serde(flatten)]
    pub(crate)  other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsontpRequest {
    pub(crate) jsontp: String,
    #[serde(rename = "type")]
    pub(crate) type_of_request: String,
    pub(crate) method: String,
    pub(crate) resource: String,
    pub(crate) headers: HashMap<String, Value>,
    pub(crate) body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Status {
    pub code: u16,
    #[serde(rename = "formal-message")]
    pub formal_message: String,
    #[serde(rename = "human-message")]
    pub human_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsontpResponse {
    pub(crate) jsontp: String,
    #[serde(rename = "type")]
    pub(crate) type_of_response: String,
    pub(crate) status: Status,
    pub(crate) resource: String,
    pub(crate) headers: HashMap<String, Value>,
    pub(crate) body: Body,
}

impl JsontpRequest {
    fn from_json(json: &str) -> Result<JsontpRequest, serde_json::Error> {
        serde_json::from_str(json)
    }

    fn validate(&self) -> Result<(), String> {
        for field in vec![
            self.jsontp.clone(),
            self.type_of_request.clone(),
            self.method.clone(),
            self.resource.clone(),
        ].iter() {
            if field.is_empty() {
                return Err(format!("Field {} is empty", field));
            }
        }

        let allowed_methods = vec!["GET", "POST", "PUT", "DELETE"];

        if !allowed_methods.contains(&self.method.as_str()) {
            return Err(format!("Method {} is not allowed", self.method));
        }

        if self.type_of_request != "request" {
            return Err(format!("Type {} is not allowed", self.type_of_request));
        }

        let allowed_encodings = vec!["gzip", "deflate", "br", "identity"];

        if !allowed_encodings.contains(&self.body.encoding.as_str()) {
            return Err(format!("Encoding {} is not allowed", self.body.encoding));
        }

        Ok(())
    }
}