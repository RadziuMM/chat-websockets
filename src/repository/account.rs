use serde::Serialize;
use std::sync::{Mutex};
use uuid::Uuid;
use crate::entity::account::Account;
use sha2::{Digest};
use constant_time_eq::constant_time_eq;

lazy_static::lazy_static! {
    #[derive(Debug, Serialize, Clone)]
    pub static ref ACCOUNTS: Mutex<Vec<Account>> = Mutex::new(vec![]);
}

pub fn is_sha256_hash(input: &str) -> bool {
    input.len() == 64 && input.chars().all(|c| c.is_digit(16))
}

pub async fn insert_account(name: String, password: String) {
    let mut accounts = ACCOUNTS.lock().unwrap();
    accounts.push(Account {
        id: Uuid::new_v4().to_string(),
        name,
        password,
    });
}

pub async fn get_account_by_id(id: String) -> Option<Account> {
    let accounts = ACCOUNTS.lock().unwrap();

    accounts.iter().find(|account| {
        account.id == id
    }).cloned()
}

pub fn match_and_return_account(name: &str, password: &str) -> Option<Account> {
    let accounts = ACCOUNTS.lock().unwrap();

    accounts.iter().find(|account| {
        account.name == name && constant_time_eq(account.password.as_bytes(), password.as_bytes())
    }).cloned()
}