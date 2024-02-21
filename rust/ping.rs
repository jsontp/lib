use std::io::Read;
use std::io::Write;
fn main() {
    let mut client = std::net::TcpStream::connect("localhost:8080").unwrap();

    let request = r#"
    {
        "jsontp": "1.0",
        "type": "request",
        "resource": "/path/to/resource",
        "method": "GET",
    
        "headers": {
            "key1": "value1"
        },
    
        "body": {
            "key1": {
                "key2": "value2"
            },
    
            "content": "raw text to be sent",
    
            "encoding": "gzip"
        }
    }
    "#;

    client.write(request.as_bytes()).unwrap();

    println!("Request sent");

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

    println!("{}", request_string);
}
