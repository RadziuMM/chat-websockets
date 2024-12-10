use tokio::io::AsyncWriteExt;
use crate::entity::request_data::RequestData;
use crate::utils::http_helper;
use crate::utils::http_helper::{is_route};

pub const PREFIX: &str = "/api/register";

pub async fn register_controller(data: RequestData) -> tokio::io::Result<()> {
    match data {
        _ if is_route("POST", "", PREFIX, &data) => create_account(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

async fn create_account(data: RequestData) -> tokio::io::Result<()> {
    let response_body = "Account created successfully!".to_string();

    let response = format!(
        "HTTP/1.1 201 CREATED\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    let mut stream = data.stream;
    stream
        .write_all(response.as_bytes())
        .await
        .expect("Error occurred on sending response");
    stream.flush().await.expect("Error occurred on flushing stream");

    Ok(())
}
