pub mod shared;
pub mod server;

pub mod prelude {
    pub use crate::shared::defs::*;
    pub use crate::server::*;

    pub use serde_json::Value;
}

#[cfg(test)]
mod tests {
    use super::*;

    use prelude::*;

    #[tokio::test]
    async fn test_server() {
            let mut server = server::Server::new("hey".to_string(), "localhost".to_string(), 8080);

            server.route(
                "/".to_string(),
                |req: JsontpRequest| {
                    let mut headers = req.headers.clone();
                    headers.insert("Content-Type".to_string(), Value::String("text/html".to_string()));
                    let body = Body::new("<h1>Hello, world!</h1>".to_string(), "identity".to_string(), None);
                    Response::new_manual(body, 200, None, "/".to_string(), Language::default())
                }
            );
            
            server.start();

    }
}
