use tokio::fs::File;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use crate::entity::request_data::RequestData;
use crate::utils::http_helper;
use crate::utils::http_helper::{close_ws_with_error, serve_static};
use crate::utils::utils::extract_path_from_request;
use crate::controller::frontend::{frontend_controller, PREFIX as FRONTEND_CONTROLLER_PREFIX};

pub async fn init(listener: TcpListener) -> std::io::Result<()> {
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                if let Err(e) = route_request(stream).await {
                    eprintln!("Error: {}", e);
                }
            });
        }
    }
}

async fn route_request(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let peeked_bytes = stream.peek(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..peeked_bytes]);
    if request.contains("Upgrade: websocket") {
        if let Some(path) = extract_path_from_request(&request) {
            match accept_async(&mut stream).await {
                Ok(ws_stream) => routing_ws(&*path, ws_stream).await,
                Err(_) => http_helper::invalid(stream).await,
            }
        } else {
            http_helper::invalid(stream).await
        }
    } else {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        if let Some(first_line) = request.lines().next() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 2 {
                return routing(RequestData {
                    stream,
                    buffer,
                    method: parts[0].to_string(),
                    path: parts[1].to_string(),
                }).await;
            }
        }
        http_helper::invalid(stream).await
    }
}

async fn routing(data: RequestData) -> Result<(), std::io::Error> {
    match &data.path {
        p if p.starts_with("/static/") => serve_static(data).await,
        p if p.starts_with(FRONTEND_CONTROLLER_PREFIX) => frontend_controller(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

async fn routing_ws(path: &str, ws_stream: WebSocketStream<&mut TcpStream>) -> Result<(), std::io::Error> {
    match path {
        // TODO: ws routing
        _ => close_ws_with_error(ws_stream, 404, "Not Found".parse().unwrap()).await
    }
}