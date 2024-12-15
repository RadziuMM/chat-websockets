use std::collections::HashMap;
use tokio::sync::{broadcast, Mutex};
use serde::Serialize;
use uuid::Uuid;
use crate::entity::message::Message;
use crate::entity::room::Room;
use once_cell::sync::Lazy;

lazy_static::lazy_static! {
    #[derive(Debug, Serialize, Clone)]
    pub static ref ROOMS: Mutex<HashMap<String, Room>> = Mutex::new(HashMap::new());
}

pub static ROOM_SENDER: Lazy<broadcast::Sender<Room>> = Lazy::new(|| {
    let (sender, _receiver) = broadcast::channel(100);
    sender
});

pub async fn create(name: String)  {
    let mut rooms = ROOMS.lock().await;

    let room = Room::new(Uuid::new_v4().to_string(), name.to_string());
    if let Err(err) = ROOM_SENDER.send(room.clone()) {
        eprintln!("Failed to broadcast room: {}", err);
    }
    rooms.insert(room.clone().id, room);
}

pub async fn get() -> Vec<Room> {
    let rooms = ROOMS.lock().await;
    rooms.values().cloned().collect()
}

pub async fn get_one_by_id(id: String) -> Option<Room> {
    let rooms = ROOMS.lock().await;
    rooms.get(&id).cloned()
}

pub async fn add_message_to_room(id: String, username: String, content: String) -> Result<Message, String> {
    let mut rooms = ROOMS.lock().await;
    if let Some(room) = rooms.get_mut(&id) {
        let message = Message {
            id: Uuid::new_v4().to_string(),
            username,
            content,
            date: chrono::Utc::now().to_rfc3339(),
        };
        room.messages.push(message.clone());
        room.sender.send(message.clone()).unwrap_or(0);
        Ok(message)
    } else {
        Err(format!("Room with id {} not found", id))
    }
}