mod controller;
mod utils;
mod entity;
mod repository;

use tokio::net::TcpListener;
use crate::controller::controller::init;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    init(listener).await.expect("Error occurred on controller init");
}
