use askama::Template;
use tokio::io::AsyncWriteExt;
use crate::entity::request_data::RequestData;
use crate::utils::http_helper;
use crate::utils::http_helper::{is_route};

pub const PREFIX: &str = "";

pub async fn frontend_controller(data: RequestData) -> tokio::io::Result<()> {
    match data {
        _ if is_route("GET", "", PREFIX, &data) => get_index(data).await,
        _ if is_route("GET", "/login", PREFIX, &data) => get_login(data).await,
        _ if is_route("GET", "/register", PREFIX, &data) => get_register(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    heading: String,
    content: String,
}

async fn get_index(data: RequestData) -> tokio::io::Result<()> {
    let template = IndexTemplate {
        title: "Hello world!".to_string(),
        heading: "Hello world!".to_string(),
        content: "ąężźć.".to_string(),
    };

    let response_body = template
        .render()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Render error"))?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
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

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

async fn get_login(data: RequestData) -> tokio::io::Result<()> {
    let template = LoginTemplate {};

    let response_body = template
        .render()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Render error"))?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
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

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {}

async fn get_register(data: RequestData) -> tokio::io::Result<()> {
    let template = RegisterTemplate {};

    let response_body = template
        .render()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Render error"))?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
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