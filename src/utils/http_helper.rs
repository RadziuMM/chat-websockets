use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use tungstenite::protocol::CloseFrame;
use futures_util::{SinkExt, StreamExt};
use tokio::fs::File;
use tokio::io;
use crate::entity::request_data::RequestData;

pub async fn not_found(stream: TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 404 NOT_FOUND\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

pub async fn invalid(stream: TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 400 INVALID_REQUEST\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

pub async fn internal_server_error(stream: TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 500 INTERNAL_SERVER_ERROR\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

async fn finish_request(stream: TcpStream, response: &str) -> std::io::Result<()> {
    let mut locked_stream = stream;
    locked_stream.write_all(response.as_bytes()).await?;
    locked_stream.flush().await?;

    Ok(())
}

pub fn is_route(method: &str, path: &str, prefix: &str, data: &RequestData) -> bool {
    let (request_path, _query) = data.path.split_once('?').unwrap_or((&data.path, ""));

    let prefix_path = format!("{}{}", prefix, path);
    let prefix_path_with_slash = format!("{}/", prefix_path);

    (request_path == prefix_path || request_path == &prefix_path_with_slash)
        && data.method == method
}

pub async fn close_ws_with_error(
    ws_stream: WebSocketStream<&mut TcpStream>,
    code: u16,
    reason: String,
) -> Result<(), io::Error> {
    let (mut sender, _) = ws_stream.split();

    let close_message = Message::Close(Some(CloseFrame {
        code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::from(code),
        reason: reason.into(),
    }));

    sender.send(close_message).await.map_err(|e| {
        eprintln!("Failed to close WebSocket connection: {}", e);
        io::Error::new(io::ErrorKind::Other, "Failed to close WebSocket")
    })
}

pub async fn serve_static(data: RequestData) -> Result<(), tokio::io::Error> {
    let file_path = format!(".{}", data.path);
    println!("Serving static file: {}", file_path);
    if Path::new(&file_path).exists() {
        let mut file = File::open(&file_path).await?;
        let mut contents = Vec::new();

        file.read_to_end(&mut contents).await?;

        let content_type = get_content_type(&file_path);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            content_type,
            contents.len()
        );

        let mut stream = data.stream;
        stream.write_all(response.as_bytes()).await?;
        stream.write_all(&contents).await?;
    } else {
        not_found(data.stream).await?;
    }

    Ok(())
}

fn get_content_type(file_path: &str) -> &str {
    if file_path.ends_with(".css") {
        "text/css"
    } else if file_path.ends_with(".js") {
        "application/javascript"
    } else if file_path.ends_with(".html") {
        "text/html"
    } else if file_path.ends_with(".png") {
        "image/png"
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg"
    } else {
        "application/octet-stream"
    }
}