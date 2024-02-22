use crate::prelude::*;

use std::{collections::HashMap, io::{Read, Write}};


pub struct Request {
    pub(crate) inner: JsontpRequest,
}


impl Request {
    pub fn new() -> Request {
        Request {
            inner: JsontpRequest {
                jsontp: "1.0-rc1".to_string(),
                type_of_request: "request".to_string(),
                method: "GET".to_string(),
                resource: "/".to_string(),
                headers: HashMap::new(),
                body: Body::new("", "identity", None),
            },
        }
    }

    pub fn method<T: ToString>(mut self, method: T) -> Request {
        self.inner.method = method.to_string();
        self
    }

    pub fn resource<T: ToString>(mut self, resource: T) -> Request {
        self.inner.resource = resource.to_string();
        self
    }

    pub fn header<T: ToString, U: ToString>(mut self, key: T, value: U) -> Request {
        self.inner.headers.insert(key.to_string(), Value::String(value.to_string()));
        self
    }

    pub fn body<T: ToString, U: ToString>(mut self, content: T, encoding: U) -> Request {
        self.inner.body = Body::new(content, encoding, None);
        self
    }

    pub fn body_key<T: ToString, U: ToString>(mut self, key: T, value: U) -> Request {
        self.inner.body.other .insert(key.to_string(), Value::String(value.to_string()));
        self
    }

    pub fn send<T: ToString>(self, host: T, port: u16) -> Result<JsontpResponse, String> {

        let mut client = std::net::TcpStream::connect(format!("{}:{}", host.to_string(), port)).unwrap();

        let request = serde_json::to_string(&self.inner).unwrap();

        client.write(request.as_bytes()).unwrap();

        let mut request_string = String::new();

        let mut buf_reader = std::io::BufReader::new(&client);

        loop {
            let mut buffer = [0; 1024];
            let bytes_read = buf_reader.read(&mut buffer).unwrap();

            request_string.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

            if bytes_read < 1024 {
                break;
            }   
        }

        match serde_json::from_str(&request_string) {
            Ok(response) => Ok(response),
            Err(e) => Err(format!("Error parsing response: {}", e)),
        }
    }
}