use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
    pub(crate) id: String,
    pub(crate) token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionTokenDTO {
    pub id: String,
    pub name: String,
    pub token: String,
}