pub(crate) mod shared;
pub mod server;
pub mod client;
mod status;

/// The prelude module re-exports the most commonly used items in this crate.
/// It should be imported in all modules that use the jsontp crate.
pub mod prelude {
    pub use crate::shared::*;
    pub use crate::server::*;

    pub use serde_json::Value;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    use prelude::*;

    #[tokio::test]
    async fn test_server() {
            let mut server = server::Server::new("hey", "localhost", 8080);

            server.route(
                "/".to_string(),
                |req: JsontpRequest| {                    
                    req.to_response(
                        Body::new("Hello, world!", "identity", None),
                        200,
                        None,
                        Language::default(),
                        None
                    )
                }
            );
            
            server.start();

        println!("hello!");
    }

    #[tokio::test]
    async fn test_client() {
        let client = client::Request::new()
            .method("GET")
            .resource("/")
            .header("key1", "value1")
            .body("raw text to be sent", "gzip");

        let response = client.send("localhost", 8080).unwrap();

        println!("Server said: {} {}", response.status, response.body.content);
    }
}
