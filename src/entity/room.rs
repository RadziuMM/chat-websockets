use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use crate::entity::message::Message;

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub messages: Vec<Message>,
    #[serde(skip)]
    pub sender: broadcast::Sender<Message>,
}

impl Room {
    pub(crate) fn new(id: String, name: String,) -> Self {
        let (sender, _receiver) = broadcast::channel(100);
        Room {
            id,
            name,
            messages: Vec::new(),
            sender,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomDTO {
    pub name: String,
}