use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    content: String,
    encoding: String,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsontpRequest {
    jsontp: String,
    #[serde(rename = "type")]
    type_of_request: String,
    method: String,
    resource: String,
    headers: HashMap<String, Value>,
    body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    code: u16,
    #[serde(rename = "formal-message")]
    formal_message: String,
    #[serde(rename = "human-message")]
    human_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsontpResponse {
    jsontp: String,
    #[serde(rename = "type")]
    type_of_response: String,
    status: Status,
    resource: String,
    headers: HashMap<String, Value>,
    body: Body,
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

        

        Ok(())
    }
}