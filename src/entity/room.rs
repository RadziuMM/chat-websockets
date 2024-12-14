use serde::{Deserialize, Serialize};
use crate::entity::message::Message;

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomDTO {
    pub name: String,
}