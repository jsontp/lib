pub(crate) mod shared;
pub mod server_imp;
pub mod client_imp;
mod status;

/// server prelude, containing all the types and traits needed to create a server
pub mod server {
    pub use crate::shared::*;
    pub use crate::server_imp::*;
    pub use crate::status::*;
    pub use serde_json::Value;
}

/// client prelude, containing all the types and traits needed to create a client
pub mod client {
    pub use crate::shared::*;
    pub use crate::client_imp::*;
    pub use crate::status::*;
    pub use serde_json::Value;
}


#[cfg(test)]
mod tests {
    use super::*;

    use server::*;

    use client::*;

    #[tokio::test]
    async fn test_server() {
            let mut server = server_imp::Server::new("hey", "localhost", 8080);

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
        let client = Request::new()
            .method("GET")
            .resource("/")
            .header("key1", "value1")
            .body("raw text to be sent", "gzip");

        let response = client.send("localhost", 8080).unwrap();

        println!("Server said: {} {}", response.status, response.body.content);
    }
}
