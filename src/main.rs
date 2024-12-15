mod controller;
mod utils;
mod entity;
mod repository;
use std::{fs};
use std::path::Path;
use rusqlite::{Connection};
use tokio::io;
use tokio::net::TcpListener;
use crate::controller::controller::init;
use crate::repository::{account, room, session};

#[tokio::main]
async fn main() {
    init_db().await.expect("Unable to create a database.");
    account::init_cache();
    room::init_cache();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    init(listener).await.expect("Error occurred on controller init");
}

pub async fn init_db() -> io::Result<()> {
    let db_path = Path::new("data.db");
    let schema_path = Path::new("schema.sql");

    if !db_path.exists() {
        println!("Database file not found. Creating and initializing...");
    } else {
        println!("Database already exists. Skipping initialization.");
    }

    if !schema_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Schema file not found at {:?}", schema_path),
        ));
    }

    let conn = Connection::open(db_path).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to open database file: {}", err),
        )
    })?;

    let schema = fs::read_to_string(schema_path).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to read schema file: {}", err),
        )
    })?;

    conn.execute_batch(&schema).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to execute schema: {}", err),
        )
    })?;

    Ok(())
}
