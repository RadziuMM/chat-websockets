use std::sync::{Mutex};
use serde::Serialize;
use uuid::Uuid;
use crate::entity::message::Message;
use crate::entity::room::Room;

lazy_static::lazy_static! {
    #[derive(Debug, Serialize, Clone)]
    pub static ref ROOMS: Mutex<Vec<Room>> = Mutex::new(vec![]);
}

pub fn create(name: String)  {
    let mut rooms = ROOMS.lock().unwrap();
    rooms.push(Room {
        id: Uuid::new_v4().to_string(),
        name,
        messages: vec![],
    });
}

pub fn get() -> Vec<Room> {
    let rooms = ROOMS.lock().unwrap();
    rooms.clone()
}

pub fn add_message_to_room(id: String, username: String, content: String) {
    let mut rooms = ROOMS.lock().unwrap();
    if let Some(room) = rooms.iter_mut().find(|ch| ch.id == id) {
        room.messages.push(Message {
            id: Uuid::new_v4().to_string(),
            username,
            content,
            date: chrono::Utc::now().to_rfc3339(),
        });
    } else {
        eprintln!("Channel with id {} not found", id);
    }
}