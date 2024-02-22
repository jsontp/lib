use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug)]
pub struct Language {
    pub(crate) lang: Option<String>,
    pub(crate) locale: Option<String>,
}

#[derive(Debug)]
pub struct Response {
    pub(crate) body: Body,
    pub(crate) status: u16, // status code, not Status struct, as the messages should not be exposed to the user to change
    pub(crate) cookies: Option<HashMap<String, String>>,
    pub(crate) resource: String,

    pub(crate) language: Language,
    pub(crate) headers: Option<HashMap<String, Value>>,
}

/// The body of a jsontp request or response, containing the content, encoding and other fields
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    pub content: String,
    pub encoding: String,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl Body {
    /// Create a new body with the given content, encoding and other fields
    pub fn new<T, U>(content: T, encoding: U, other: Option<HashMap<String, Value>>) -> Body
    where
        T: ToString,
        U: ToString,
    {
        Body {
            content: content.to_string(),
            encoding: encoding.to_string(),
            other: match other {
                Some(map) => map,
                None => HashMap::new(),
            },
        }
    }
}

/// The jsontp request, containing the jsontp version, specified by the standard
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

/// The status of a jsontp response, containing the code, formal message and human message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub code: u16,
    #[serde(rename = "formal-message")]
    pub formal_message: String,
    #[serde(rename = "human-message")]
    pub human_message: String,
}

impl core::fmt::Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.code, self.formal_message)
    }
}

/// The jsontp response, specified by the standard
#[derive(Serialize, Deserialize, Debug)]
pub struct JsontpResponse {
    pub jsontp: String,
    #[serde(rename = "type")]
    pub(crate) type_of_response: String,
    pub status: Status,
    pub resource: String,
    pub headers: HashMap<String, Value>,
    pub body: Body,
}

impl JsontpRequest {
    pub(crate) fn validate(&self) -> Result<(), String> {
        for field in vec![
            self.jsontp.clone(),
            self.type_of_request.clone(),
            self.method.clone(),
            self.resource.clone(),
        ]
        .iter()
        {
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

    pub fn to_response(
        &self,
        body: Body,
        status: u16,
        cookies: Option<HashMap<String, String>>,
        language: Language,
        headers: Option<HashMap<String, Value>>,
    ) -> Response {
        if let Some(langs) = self.headers.get("accept-language") {
            match langs {
                Value::String(s) => {
                    if &language.to_string() != s && !s.contains(&language.lang.clone().unwrap_or_default()) && !s.contains("*") {
                        return Response::new_manual(
                            Body::new("Language not supported".to_string(), "identity", None),
                            406,
                            None,
                            self.resource.clone(),
                            language,
                            None,
                        );
                    }
                }
                Value::Array(arr) => {
                    let mut found = false;
                    for lang in arr {
                        if let Value::String(s) = lang {
                            if &language.to_string() == s || s.contains(&language.lang.clone().unwrap_or_default()) || s.contains("*") {
                                found = true;
                                break;
                            }
                        }
                    }

                    if !found {
                        return Response::new_manual(
                            Body::new("Language not supported".to_string(), "identity", None),
                            406,
                            None,
                            self.resource.clone(),
                            language,
                            None,
                        );
                    }
                }
                _ => {
                    return Response::new_manual(
                        Body::new("Language not supported".to_string(), "identity", None),
                        406,
                        None,
                        self.resource.clone(),
                        language,
                        None,
                    );
                }
            }
        }

        match self.validate() {
            Ok(_) => Response::new_manual(
                body,
                status,
                cookies.map(|map| {
                    map.into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect()
                }),
                self.resource.clone(),
                language,
                headers,
            ),
            Err(e) => {
                return Response::new_manual(
                    Body::new(e, "identity", None),
                    400,
                    None,
                    self.resource.clone(),
                    language,
                    None,
                );
            }
        }
    }
}
