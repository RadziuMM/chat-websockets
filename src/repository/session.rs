use serde::Serialize;
use std::sync::{Mutex};
use uuid::Uuid;
use crate::entity::session::Session;

lazy_static::lazy_static! {
    #[derive(Debug, Serialize, Clone)]
    pub static ref SESSIONS: Mutex<Vec<Session>> = Mutex::new(vec![]);
}

pub fn create_session(id: String) -> Session {
    let mut sessions = SESSIONS.lock().unwrap();
    let session = Session {
        id,
        token: Uuid::new_v4().to_string(),
    };

    sessions.push(session.clone());
    session
}

pub fn match_and_return_session(id: String, token: String) -> Option<Session> {
    let sessions = SESSIONS.lock().unwrap();

    sessions.iter().find(|session| {
        session.id == id && session.token == token
    }).cloned()
}

pub fn stop_session(id: &str) -> bool {
    let mut sessions = SESSIONS.lock().unwrap();

    let initial_length = sessions.len();
    sessions.retain(|session| session.id != id);
    let final_length = sessions.len();

    initial_length != final_length
}