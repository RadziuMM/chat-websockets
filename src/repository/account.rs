use serde::Serialize;
use std::sync::{Mutex};
use uuid::Uuid;
use crate::entity::account::Account;
use constant_time_eq::constant_time_eq;
use rusqlite::{Connection};

lazy_static::lazy_static! {
    #[derive(Debug, Serialize, Clone)]
    pub static ref ACCOUNTS: Mutex<Vec<Account>> = Mutex::new(vec![]);
    pub static ref DB: Mutex<Connection> = Mutex::new(
        Connection::open("data.db").expect("Failed to connect to SQLite")
    );
}

pub fn init_cache() {
    let conn = DB.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, password FROM accounts;")
        .expect("Failed to prepare statement");

    let account_iter = stmt
        .query_map([], |row| {
            Ok(Account {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .expect("Failed to query all accounts");

    let mut accounts = ACCOUNTS.lock().unwrap();
    for account in account_iter.flatten() {
        accounts.push(account);
    }
}

pub fn is_sha256_hash(input: &str) -> bool {
    input.len() == 64 && input.chars().all(|c| c.is_digit(16))
}

pub async fn insert_account(name: String, password: String) {
    let account = Account {
        id: Uuid::new_v4().to_string(),
        name: name.clone(),
        password: password.clone(),
    };

    {
        let conn = DB.lock().unwrap();
        conn.execute(
            "INSERT INTO accounts (id, name, password) VALUES (?1, ?2, ?3);",
            &[&account.id, &account.name, &account.password],
        )
            .expect("Failed to insert account into database");
    }

    let mut accounts = ACCOUNTS.lock().unwrap();
    accounts.push(account);
}

pub async fn get_account_by_id(id: String) -> Option<Account> {
    let accounts = ACCOUNTS.lock().unwrap();
    if let Some(account) = accounts.iter().find(|account| account.id == id).cloned() {
        return Some(account);
    }
    drop(accounts);

    let conn = DB.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, password FROM accounts WHERE id = ?1;")
        .expect("Failed to prepare statement");
    let account_iter = stmt
        .query_map([id], |row| {
            Ok(Account {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .expect("Failed to query account by id");

    if let Some(account) = account_iter.flatten().next() {
        let mut accounts = ACCOUNTS.lock().unwrap();
        accounts.push(account.clone());
        return Some(account);
    }
    None
}

pub fn match_and_return_account(name: &str, password: &str) -> Option<Account> {
    {
        let accounts = ACCOUNTS.lock().unwrap();
        if let Some(account) = accounts.iter().find(|account| {
            account.name == name && constant_time_eq(account.password.as_bytes(), password.as_bytes())
        }) {
            return Some(account.clone());
        }
    }

    let conn = DB.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, password FROM accounts WHERE name = ?1;")
        .expect("Failed to prepare statement");
    let account_iter = stmt
        .query_map([name], |row| {
            Ok(Account {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .expect("Failed to query account by name");

    for account in account_iter.flatten() {
        if constant_time_eq(account.password.as_bytes(), password.as_bytes()) {
            let mut accounts = ACCOUNTS.lock().unwrap();
            accounts.push(account.clone());
            return Some(account);
        }
    }
    None
}