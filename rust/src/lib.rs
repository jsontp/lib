pub mod shared;
pub mod server;

use std::io::{Read, Write};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server() {
            let server = server::Server::new("hey".to_string(), "localhost".to_string(), 8080);

            server.start();

        println!("hello!");


    }
}
