use std::collections::HashMap;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use tungstenite::protocol::CloseFrame;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::fs::File;
use tokio::io;
use crate::entity::request_data::RequestData;

pub async fn ok(stream: TcpStream) -> io::Result<()> {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

pub async fn not_found(stream: TcpStream) -> io::Result<()> {
    let response = "HTTP/1.1 404 NOT_FOUND\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

pub async fn invalid(stream: TcpStream) -> io::Result<()> {
    let response = "HTTP/1.1 400 INVALID_REQUEST\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

pub async fn unauthenticated(stream: TcpStream) -> io::Result<()> {
    let response = "HTTP/1.1 401 UNAUTHENTICATED\r\nContent-Length: 0\r\n\r\n";
    finish_request(stream, response).await
}

pub async fn finish_request(stream: TcpStream, response: &str) -> io::Result<()> {
    let mut locked_stream = stream;
    locked_stream.write_all(response.as_bytes()).await?;
    locked_stream.flush().await?;

    Ok(())
}

pub fn is_route(method: &str, path: &str, prefix: &str, data: &mut RequestData) -> bool {
    let (request_path, _query) = data.path.split_once('?').unwrap_or((&data.path, ""));

    if path.is_empty() {
        return (request_path == prefix || request_path == format!("{}/", prefix))
            && data.method == method;
    }

    let full_path = if prefix.ends_with('/') || path.starts_with('/') {
        format!("{}{}", prefix, path)
    } else {
        format!("{}/{}", prefix, path)
    };

    let expected_segments: Vec<&str> = full_path.split('/').collect();
    let request_segments: Vec<&str> = request_path.split('/').collect();

    if data.method != method {
        return false;
    }

    if expected_segments.len() != request_segments.len() {
        return false;
    }

    let normalized_expected = expected_segments
        .iter()
        .map(|&segment| if segment.starts_with(':') { "" } else { segment })
        .collect::<Vec<_>>()
        .join("/");


    let normalized_request = request_segments
        .iter()
        .zip(expected_segments.iter())
        .map(|(&req_segment, &expected_segment)| {
            if expected_segment.starts_with(':') {
                ""
            } else {
                req_segment
            }
        })
        .collect::<Vec<_>>()
        .join("/");

    data.params.clear();
    for (expected_segment, request_segment) in expected_segments.iter().zip(request_segments.iter())
    {
        if expected_segment.starts_with(':') {
            let key = expected_segment.trim_start_matches(':').to_string();
            data.params.insert(key, request_segment.to_string());
        } else if expected_segment != request_segment {
            return false;
        }
    }

    normalized_request == normalized_expected
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

pub async fn serve_static(data: RequestData) -> Result<(), io::Error> {
    let file_path = format!(".{}", data.path);
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

pub fn parse_body<T: for<'de> Deserialize<'de>>(buffer: &[u8]) -> Result<T, String> {
    let content = std::str::from_utf8(buffer).map_err(|_| "Invalid UTF-8 encoding".to_string())?;
    let body_start = content.find("\r\n\r\n")
        .ok_or_else(|| "Invalid HTTP format: Missing body".to_string())? + 4;
    let body = content[body_start..].trim_end_matches('\0').trim();
    serde_json::from_str(body).map_err(|err| format!("Failed to parse JSON: {}", err))
}

pub fn parse_cookies(headers: &str) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.lines().find(|line| line.starts_with("Cookie:")) {
        if let Some(cookie_str) = cookie_header.strip_prefix("Cookie: ") {
            for cookie in cookie_str.split(';') {
                let mut parts = cookie.splitn(2, '=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    cookies.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }
    }
    cookies
}

pub fn is_ws_route(path: &str, prefix: &str, data_path: &str) -> bool {
    let (request_path, _query) = data_path.split_once('?').unwrap_or((&data_path, ""));

    let prefix_path = format!("{}{}", prefix, path);
    let prefix_path_with_slash = format!("{}/", prefix_path);

    request_path == prefix_path || request_path == &prefix_path_with_slash
}

pub fn get_query_params(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            let key = parts.next()?;
            let value = parts.next()?;
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}