use crate::shared::defs::*;

use std::collections::HashMap;

use serde_json::{Value, self};

use std::io::{Read, Write};

#[derive(Debug)]
pub struct Response {
    body: HashMap<String, Value>,
    status: u16,
    cookies: HashMap<String, String>,
    resource: String,
}

impl Response {
    pub fn new(
        body: HashMap<String, Value>,
        status: u16,
        cookies: HashMap<String, String>,
        resource: String
    ) -> Response {
        Response {
            body,
            status,
            cookies,
            resource,
        }
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.body.is_empty() {
            return Err("Body is empty".to_string());
        }

        if self.status < 100 || self.status > 599 {
            return Err("Status code is not in the range 100-599".to_string());
        }

        if !self.body.contains_key("content"){
            return Err("Body does not contain a content field".to_string());
        }

        if !self.body.contains_key("encoding"){
            return Err("Body does not contain an encoding field".to_string());
        }

        if self.body.get("encoding").unwrap_or(&Value::Null).as_str().unwrap_or("").is_empty() {
            return Err("Body encoding is empty".to_string());
        }

        let allowed_encodings = vec!["gzip", "deflate", "br", "identity"];

        if !allowed_encodings.contains(&self.body.get("encoding").unwrap_or(&Value::Null).as_str().unwrap_or("".to_string().as_str())) {
            return Err("Body encoding is not allowed".to_string());
        }

        Ok(())
    }

    fn to_jsontp_response(&self) -> JsontpResponse {
        let validation = self.validate();

        let status = match validation {
            Ok(_) => Status {
                code: self.status,
                formal_message: "OK".to_string(),
                human_message: "Request was successful".to_string(),
            },
            Err(message) => Status {
                code: 400,
                formal_message: "Bad Request".to_string(),
                human_message: message,
            },
        };

        JsontpResponse {
            jsontp: "1.0-rc1".to_string(),
            type_of_response: "response".to_string(),
            status,
            resource: self.resource.clone(),
            headers: HashMap::new(),
            body: Body {
                content: self.body.get("content").unwrap().clone().to_string(),
                encoding: self.body.get("encoding").unwrap().clone().to_string(),
                other: HashMap::new(),
            },
        }
    }
}

#[derive(Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub version: String,
    pub port: u16,
    pub route_handlers: HashMap<String, fn(JsontpRequest) -> Response>,
    pub error_handlers: HashMap<u16, fn(JsontpRequest) -> Response>,
}

impl Server {
    pub fn new(name: String, host: String, port: u16) -> Server {
        Server {
            name,
            host,
            version: "1.0-rc1".to_string(),
            port,
            route_handlers: HashMap::new(),
            error_handlers: HashMap::new(),
        }
    }

    pub fn route(&mut self, route: String, handler: fn(JsontpRequest) -> Response) {
        self.route_handlers.insert(route, handler);
    }

    pub fn add_error_handler(&mut self, code: u16, handler: fn(JsontpRequest) -> Response) {
        self.error_handlers.insert(code, handler);
    }

    pub fn start(self) {
        let listener = std::net::TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();

        for stream in listener.incoming() {

            let server = self.clone();

            std::thread::spawn(move || {

                if let Err(e) = stream {
                    eprintln!("failed: {}", e);
                    return;
                }

                let mut stream = stream.unwrap();

                println!("Connection established!");

                let mut request_string = String::new();

                let mut buf_reader = std::io::BufReader::new(&stream);


                loop {
                    let mut buffer = [0; 1024];
                    let bytes_read = buf_reader.read(&mut buffer).unwrap();

                    request_string.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

                    if bytes_read < 1024 {
                        break;
                    }   
                }


                println!("{}", request_string);

                let request: JsontpRequest = serde_json::from_str(&request_string).unwrap();

                let mut response = match server.route_handlers.get(&request.resource) {
                    Some(handler) => handler(request).to_jsontp_response(),
                    None => JsontpResponse {
                        jsontp: "1.0-rc1".to_string(),
                        type_of_response: "response".to_string(),
                        status: Status {
                            code: 404,
                            formal_message: "Not Found".to_string(),
                            human_message: "Resource not found".to_string(),
                        },
                        resource: request.resource.clone(),
                        headers: HashMap::new(),
                        body: Body {
                            content: "".to_string(),
                            encoding: "".to_string(),
                            other: HashMap::new(),
                        },
                    },
                };

                let response_string = serde_json::to_string(&response).unwrap();

                stream.write(response_string.as_bytes()).unwrap();
            }
            );
        }
    }
}