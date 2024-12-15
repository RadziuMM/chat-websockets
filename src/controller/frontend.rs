use askama::Template;
use crate::entity::request_data::RequestData;
use crate::entity::template::{IndexTemplate, RegisterTemplate, LoginTemplate, LayoutTemplate, RoomTemplate};
use crate::repository::account::get_account_by_id;
use crate::utils::http_helper;
use crate::utils::http_helper::{finish_request, is_route};
use crate::utils::utils::{authorize, clear_cookies_response};

pub const PREFIX: &str = "";

pub async fn frontend_controller(mut data: RequestData) -> tokio::io::Result<()> {
    match data {
        _ if is_route("GET", "", PREFIX, &mut data) => get_index(data).await,
        _ if is_route("GET", "/login", PREFIX, &mut data) => get_login(data).await,
        _ if is_route("GET", "/register", PREFIX, &mut data) => get_register(data).await,
        _ if is_route("GET", "/room", PREFIX, &mut data) => get_room(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

async fn get_index(data: RequestData) -> tokio::io::Result<()> {
    let session = match authorize(&data.buffer) {
        Ok(data) => data,
        Err(_) => {
            let response = format!("HTTP/1.1 302 Found\r\n\
                Location: /login\r\n\
                {}
                \r\n",
                clear_cookies_response()
            );

            return finish_request(data.stream, &response).await
        },
    };

    let _ = get_account_by_id(session.id);

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

async fn get_room(data: RequestData) -> tokio::io::Result<()> {
    let session = match authorize(&data.buffer) {
        Ok(data) => data,
        Err(_) => {
            let response = format!("HTTP/1.1 302 Found\r\n\
                Location: /login\r\n\
                {}
                \r\n",
               clear_cookies_response()
            );

            return finish_request(data.stream, &response).await
        },
    };

    let _ = get_account_by_id(session.id);

    let template = LayoutTemplate {
        child: RoomTemplate {},
        subtitle: " - room".to_string(),
        js: "room.js".to_string(),
        css: "room.css".to_string(),
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