use askama::Template;
use crate::entity::request_data::RequestData;
use crate::entity::template::{IndexTemplate, RegisterTemplate, LoginTemplate, LayoutTemplate};
use crate::repository::account::get_account_by_id;
use crate::utils::http_helper;
use crate::utils::http_helper::{finish_request, invalid, is_route, parse_cookies};
use crate::utils::utils::authorize;

pub const PREFIX: &str = "";

pub async fn frontend_controller(data: RequestData) -> tokio::io::Result<()> {
    match data {
        _ if is_route("GET", "", PREFIX, &data) => get_index(data).await,
        _ if is_route("GET", "/login", PREFIX, &data) => get_login(data).await,
        _ if is_route("GET", "/register", PREFIX, &data) => get_register(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

async fn get_index(data: RequestData) -> tokio::io::Result<()> {
    let session = match authorize(&data.buffer) {
        Ok(data) => data,
        Err(_) => {
            let response = "HTTP/1.1 302 Found\r\n\
                 Location: /login\r\n\
                 Set-Cookie: id=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict\r\n\
                 Set-Cookie: token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict\r\n\
                 Set-Cookie: name=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict\r\n\
                 \r\n".to_string();

            return finish_request(data.stream, &response).await
        },
    };

    let account = get_account_by_id(session.id);

    let template = LayoutTemplate {
        child: IndexTemplate {},
        subtitle: "".to_string(),
        js: "index.js".to_string(),
        css: "index.css".to_string(),
    };

    let response_body = template
        .render()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Render error"))?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    finish_request(data.stream, &*response).await
}

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

    finish_request(data.stream, &*response).await
}

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

    finish_request(data.stream, &*response).await
}