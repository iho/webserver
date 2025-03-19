use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: Vec<(String, String)>,
}

impl HttpRequest {
    fn parse(request: &str) -> Result<Self, Box<dyn Error>> {
        let mut lines = request.lines();
        let request_line = lines.next().ok_or("Empty request")?;
        
        let mut parts = request_line.split_whitespace();
        let method = parts.next().ok_or("No method")?.to_string();
        let path = parts.next().ok_or("No path")?.to_string();
        let version = parts.next().ok_or("No version")?.to_string();

        let mut headers = Vec::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
        })
    }
}

async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let n = socket.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..n]);

    let http_request = HttpRequest::parse(&request)?;
    println!("Received request: {:?}", http_request);

    // Send HTTP response
    let response = format!(
        "HTTP/1.0 200 OK\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: 13\r\n\
         \r\n\
         Hello, World!"
    );

    socket.write_all(response.as_bytes()).await?;
    socket.flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
