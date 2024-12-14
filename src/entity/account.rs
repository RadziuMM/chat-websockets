use serde::Deserialize;

#[derive(Clone)]
pub struct Account {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDTO {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct MeDTO {
    pub id: String,
    pub token: String,
}