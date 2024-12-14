use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub username: String,
    pub content: String,
    pub date: String,
}