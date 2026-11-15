
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).unwrap();
    println!("CROWN V11 listening on {}", addr);

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0; 2048];
            let _ = stream.read(&mut buffer);
            let request = String::from_utf8_lossy(&buffer);

            let (status, body) = if request.contains("GET /health") {
                ("200 OK", "{\"status\":\"ok\",\"node\":\"CROWN-V11\"}")
            } else if request.contains("GET /") {
                ("200 OK", "{\"crown\":\"V11\",\"status\":\"GREEN WOLVES NODE LIVE\"}")
            } else {
                ("404 NOT FOUND", "{\"error\":\"not found\"}")
            };

            let response = format!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                status,
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
}
