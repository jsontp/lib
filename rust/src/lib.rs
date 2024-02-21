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

    }
}
