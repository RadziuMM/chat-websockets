use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use crate::entity::request_data::RequestData;
use crate::entity::room::{CreateRoomDTO};
use crate::repository::account::get_account_by_id;
use crate::repository::room;
use crate::repository::room::{add_message_to_room, ROOMS, ROOM_SENDER};
use crate::utils::http_helper::{finish_request, get_query_params, invalid, is_route, is_ws_route, not_found, ok, parse_body};
use crate::utils::utils::authorize;

pub const PREFIX: &str = "/api/room";

pub async fn room_controller(mut data: RequestData) -> tokio::io::Result<()> {
    match data {
        _ if is_route("GET", "", PREFIX, &mut data) => get_rooms(data).await,
        _ if is_route("GET", ":id", PREFIX, &mut data) => get_room(data).await,
        _ if is_route("POST", "", PREFIX, &mut data) => create_room(data).await,
        _ => not_found(data.stream).await
    }
}

pub async fn router_room_ws(path: &str, ws_stream: WebSocketStream<&mut TcpStream>, buffer: [u8; 1024]) -> std::io::Result<()> {
    match path {
        _ if is_ws_route("/message/send", PREFIX, path) => send_message(ws_stream, path, buffer).await,
        _ if is_ws_route("/message/get", PREFIX, path) => receive_message(ws_stream, path, buffer).await,
        _ if is_ws_route("/get", PREFIX, path) => receive_room(ws_stream, path, buffer).await,
        _ if is_ws_route("/send", PREFIX, path) => send_room(ws_stream, path, buffer).await,
        _ => Err(tokio::io::Error::new(tokio::io::ErrorKind::NotFound, "Route not found")),
    }
}

async fn create_room(data: RequestData) -> tokio::io::Result<()> {
    let body: CreateRoomDTO = match parse_body(&data.buffer) {
        Ok(data) => data,
        Err(_) => return invalid(data.stream).await,
    };

    room::create(body.name).await;

    ok(data.stream).await
}

async fn get_rooms(data: RequestData) -> tokio::io::Result<()> {
    let rooms = room::get().await;
    let response_body = serde_json::to_string(&rooms)?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    finish_request(data.stream, &response).await
}

async fn get_room(data: RequestData) -> tokio::io::Result<()> {
    if let Some(id) = data.params.get("id") {
        let room = room::get_one_by_id(id.to_string()).await;
        let response_body = serde_json::to_string(&room)?;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response_body.len(),
            response_body
        );

        return finish_request(data.stream, &response).await
    }

    not_found(data.stream).await
}

async fn send_message(ws_stream: WebSocketStream<&mut TcpStream>, path: &str, buffer: [u8; 1024]) -> tokio::io::Result<()> {
    let (mut sender, mut receiver) = ws_stream.split();
    let (_, query) = path.split_once('?').unwrap_or((&path, ""));
    let params = get_query_params(query);

    let session = match authorize(&buffer) {
        Ok(data) => data,
        Err(_) => {
            return Ok(())
        },
    };

    let account = get_account_by_id(session.id).await;
    let id = params.get("id").cloned().unwrap_or_default();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            tungstenite::Message::Text(text) => {
                match add_message_to_room(
                    id.clone(),
                    account.clone().unwrap().name,
                    text.clone()
                ).await {
                    Ok(_) => {}
                    Err(err) => {
                        sender
                            .send(tungstenite::Message::Text(format!(
                                "Error: {}",
                                err
                            )))
                            .await
                            .expect("Error occurred on sending error message");
                    }
                }
            }
            tungstenite::Message::Close(_) => break,
            _ => println!("Received unexpected message"),

        }
    }

    Ok(())
}

async fn receive_message(mut ws_stream: WebSocketStream<&mut TcpStream>, path: &str, buffer: [u8; 1024]) -> tokio::io::Result<()> {
    let (_, query) = path.split_once('?').unwrap_or((&path, ""));
    let params = get_query_params(query);

    let room_id = params.get("id").cloned().unwrap_or_default();
    let room = {
        let rooms = ROOMS.lock().await;
        rooms.get(&room_id).cloned().ok_or_else(|| {
            tokio::io::Error::new(tokio::io::ErrorKind::NotFound, "Room not found")
        })?
    };

    let mut broadcast_receiver = room.sender.subscribe();

    while let Ok(message) = broadcast_receiver.recv().await {
        if ws_stream
            .send(tungstenite::Message::Text(
                serde_json::to_string(&message).unwrap(),
            ))
            .await
            .is_err()
        {
            break;
        }
    }

    Ok(())
}

async fn send_room(ws_stream: WebSocketStream<&mut TcpStream>, path: &str, buffer: [u8; 1024]) -> tokio::io::Result<()> {
    let (_, mut receiver) = ws_stream.split();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            tungstenite::Message::Text(text) => {
                room::create(text.clone()).await;
            }
            tungstenite::Message::Close(_) => break,
            _ => println!("Received unexpected message"),

        }
    }

    Ok(())
}

async fn receive_room(mut ws_stream: WebSocketStream<&mut TcpStream>, path: &str, buffer: [u8; 1024]) -> tokio::io::Result<()> {
    let mut broadcast_receiver = ROOM_SENDER.subscribe();
    while let Ok(_) = broadcast_receiver.recv().await {
        if ws_stream
            .send(tungstenite::Message::Text("update".to_string()))
            .await
            .is_err()
        {
            break;
        }
    }

    Ok(())
}