use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use crate::repository::account::get_account_by_id;
use crate::repository::room::{add_message_to_room, ROOMS};
use crate::utils::http_helper::{get_query_params, is_ws_route};
use crate::utils::utils::authorize;

pub const PREFIX: &str = "/api/message";

pub async fn router_message_ws(path: &str, ws_stream: WebSocketStream<&mut TcpStream>, buffer: [u8; 1024]) -> std::io::Result<()> {
    match path {
        _ if is_ws_route("/send", PREFIX, path) => send_message(ws_stream, path, buffer).await,
        _ if is_ws_route("/get", PREFIX, path) => receive_message(ws_stream, path, buffer).await,
        _ => Err(tokio::io::Error::new(tokio::io::ErrorKind::NotFound, "Route not found")),
    }
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