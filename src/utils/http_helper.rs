
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use std::io;
use tokio::net::TcpStream;
use tungstenite::protocol::CloseFrame;
use futures_util::{SinkExt, StreamExt};
use crate::entity::request_data::RequestData;

pub async fn not_found(stream: TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 404 NOT_FOUND\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

pub async fn invalid(stream: TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 400 INVALID_REQUEST\r\nContent-Length: 0\r\n\r\n";
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