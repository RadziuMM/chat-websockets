use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::{broadcast};
use serde::Serialize;
use uuid::Uuid;
use crate::entity::message::Message;
use crate::entity::room::Room;
use once_cell::sync::Lazy;
use rusqlite::Connection;

lazy_static::lazy_static! {
    #[derive(Debug, Serialize, Clone)]
    pub static ref ROOMS: Mutex<HashMap<String, Room>> = Mutex::new(HashMap::new());
    pub static ref DB: Mutex<Connection> = Mutex::new(
        Connection::open("data.db").expect("Failed to connect to SQLite")
    );
}

pub static ROOM_SENDER: Lazy<broadcast::Sender<Room>> = Lazy::new(|| {
    let (sender, _receiver) = broadcast::channel(100);
    sender
});

pub fn init_cache() {
    let conn = DB.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name FROM rooms;")
        .expect("Failed to prepare statement");

    let room_iter = stmt
        .query_map([], |row| {
            Ok(Room {
                id: row.get(0)?,
                name: row.get(1)?,
                messages: vec![],
                sender: broadcast::channel(100).0, // Każdy pokój ma własny kanał
            })
        })
        .expect("Failed to query rooms");

    let mut rooms = ROOMS.lock().unwrap();

    for mut room in room_iter.flatten() {
        let mut message_stmt = conn
            .prepare("SELECT id, username, content, date FROM messages WHERE room_id = ?;")
            .expect("Failed to prepare statement for messages");

        let message_iter = message_stmt
            .query_map([room.id.clone()], |row| {
                Ok(Message {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    content: row.get(2)?,
                    date: row.get(3)?,
                })
            })
            .expect("Failed to query messages");

        room.messages = message_iter.flatten().collect();

        rooms.insert(room.id.clone(), room);
    }
}

pub async fn create(name: String) {
    let room = Room::new(Uuid::new_v4().to_string(), name.clone());

    {
        let conn = DB.lock().unwrap();
        conn.execute(
            "INSERT INTO rooms (id, name) VALUES (?1, ?2);",
            &[&room.id, &room.name],
        )
            .expect("Failed to insert room into database");
    }

    let mut rooms = ROOMS.lock().unwrap();
    if let Err(err) = ROOM_SENDER.send(room.clone()) {
        eprintln!("Failed to broadcast room: {}", err);
    }
    rooms.insert(room.id.clone(), room);
}

pub async fn delete(id: String) {
    {
        let conn = DB.lock().unwrap();
        conn.execute("DELETE FROM rooms WHERE id = ?1;", [&id])
            .expect("Failed to delete room from database");
        conn.execute("DELETE FROM messages WHERE room_id = ?1;", [&id])
            .expect("Failed to delete messages from database");
    }

    let mut rooms = ROOMS.lock().unwrap();
    if let Some(room) = rooms.remove(&id) {
        if let Err(err) = ROOM_SENDER.send(room.clone()) {
            eprintln!("Failed to broadcast room: {}", err);
        }
    }
}

pub async fn get() -> Vec<Room> {
    let mut rooms = ROOMS.lock().unwrap();
    if !rooms.is_empty() {
        return rooms.values().cloned().collect();
    }

    let conn = DB.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name FROM rooms;")
        .expect("Failed to prepare statement");

    let room_iter = stmt
        .query_map([], |row| {
            Ok(Room {
                id: row.get(0)?,
                name: row.get(1)?,
                messages: vec![],
                sender: broadcast::channel(100).0,
            })
        })
        .expect("Failed to query rooms");

    let mut new_rooms = vec![];
    for room in room_iter.flatten() {
        new_rooms.push(room.clone());
        rooms.insert(room.id.clone(), room);
    }

    new_rooms
}

pub async fn get_one_by_id(id: String) -> Option<Room> {
    {
        let rooms = ROOMS.lock().unwrap();
        if let Some(room) = rooms.get(&id) {
            return Some(room.clone());
        }
    }

    let conn = DB.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name FROM rooms WHERE id = ?1;")
        .expect("Failed to prepare statement");

    let mut room_iter = stmt
        .query_map([id.clone()], |row| {
            Ok(Room {
                id: row.get(0)?,
                name: row.get(1)?,
                messages: vec![],
                sender: broadcast::channel(100).0,
            })
        })
        .expect("Failed to query room by id");

    if let Some(Ok(room)) = room_iter.next() {
        let mut messages_stmt = conn
            .prepare("SELECT id, username, content, date FROM messages WHERE room_id = ?1;")
            .expect("Failed to prepare statement for messages");

        let message_iter = messages_stmt
            .query_map([id.clone()], |row| {
                Ok(Message {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    content: row.get(2)?,
                    date: row.get(3)?,
                })
            })
            .expect("Failed to query messages");

        let mut messages = vec![];
        for message in message_iter.flatten() {
            messages.push(message);
        }

        let mut room = room.clone();
        room.messages = messages;

        let mut rooms = ROOMS.lock().unwrap();
        rooms.insert(id.clone(), room.clone());

        return Some(room);
    }

    None
}

pub async fn add_message_to_room(id: String, username: String, content: String) -> Result<Message, String> {
    let mut rooms = ROOMS.lock().unwrap();

    if let Some(room) = rooms.get_mut(&id) {
        let message = Message {
            id: Uuid::new_v4().to_string(),
            username: username.clone(),
            content: content.clone(),
            date: chrono::Utc::now().to_rfc3339(),
        };

        {
            let conn = DB.lock().unwrap();
            conn.execute(
                "INSERT INTO messages (id, room_id, username, content, date) VALUES (?1, ?2, ?3, ?4, ?5);",
                &[&message.id, &id, &username, &content, &message.date],
            )
                .expect("Failed to insert message into database");
        }

        room.messages.push(message.clone());
        room.sender.send(message.clone()).unwrap_or(0);

        Ok(message)
    } else {
        Err(format!("Room with id {} not found", id))
    }
}