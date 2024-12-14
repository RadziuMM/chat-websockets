use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use crate::entity::request_data::RequestData;
use crate::entity::room::CreateRoomDTO;
use crate::repository::room;
use crate::repository::room::add_message_to_room;
use crate::utils::http_helper;
use crate::utils::http_helper::{finish_request, get_query_params, invalid, is_route, is_ws_route, ok, parse_body};

pub const PREFIX: &str = "/api/room";

pub async fn room_controller(data: RequestData) -> tokio::io::Result<()> {
    match data {
        _ if is_route("GET", "", PREFIX, &data) => get_room(data).await,
        _ if is_route("POST", "", PREFIX, &data) => create_room(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

pub async fn router_room_ws(path: &str, ws_stream: WebSocketStream<&mut TcpStream>) -> std::io::Result<()> {
    match path {
        _ if is_ws_route("", PREFIX, path) => send_to_room(ws_stream, path).await,
        _ => Err(tokio::io::Error::new(tokio::io::ErrorKind::NotFound, "Route not found")),
    }
}

async fn create_room(data: RequestData) -> tokio::io::Result<()> {
    let body: CreateRoomDTO = match parse_body(&data.buffer) {
        Ok(data) => data,
        Err(_) => return invalid(data.stream).await,
    };

    room::create(body.name);

    ok(data.stream).await
}

async fn get_room(data: RequestData) -> tokio::io::Result<()> {
    let channels = room::get();
    let response_body = serde_json::to_string(&*channels)?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    finish_request(data.stream, &response).await
}

async fn send_to_room(ws_stream: WebSocketStream<&mut TcpStream>, path: &str) -> tokio::io::Result<()> {
    let (mut sender, mut receiver) = ws_stream.split();
    let (_, query) = path.split_once('?').unwrap_or((&path, ""));
    let params = get_query_params(query);

    let id = params.get("id").cloned().unwrap_or_default();
    let name = params.get("name").cloned().unwrap_or_default();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            tungstenite::Message::Text(text) => add_message_to_room(id.clone(), name.clone(), text),
            tungstenite::Message::Binary(_) => {
                sender.send(tungstenite::Message::Text("Binary messages not supported".to_string()))
                    .await
                    .expect("Error occurred on binary request");
            }
            tungstenite::Message::Close(_) => break,
            _ => println!("Received unexpected message"),

        }
    }

    Ok(())
}


