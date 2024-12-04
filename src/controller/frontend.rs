use tokio::io::AsyncWriteExt;
use crate::entity::request_data::RequestData;
use crate::utils::http_helper;
use crate::utils::http_helper::{is_route};

pub const PREFIX: &str = "";

pub async fn frontend_controller(data: RequestData) -> std::io::Result<()> {
    match data {
        _ if is_route("GET", "", PREFIX, &data) => get_index(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

async fn get_index(data: RequestData) -> std::io::Result<()> {
    let response_body = "Hello World!";

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    let mut stream = data.stream;
    stream.write_all(response.as_bytes()).await.expect("Error occurred on closing request");
    stream.flush().await.expect("Error occurred on closing request");
    Ok(())
}