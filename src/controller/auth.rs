use crate::entity::account::{LoginDTO, MeDTO, RegisterDTO};
use crate::entity::request_data::RequestData;
use crate::entity::session::SessionTokenDTO;
use crate::repository::account::{is_sha256_hash, insert_account, match_and_return_account, get_account_by_id};
use crate::repository::session::{create_session, match_and_return_session};
use crate::utils::http_helper;
use crate::utils::http_helper::{finish_request, invalid, is_route, parse_body, unauthenticated};

pub const PREFIX: &str = "/api/auth";

pub async fn register_controller(data: RequestData) -> tokio::io::Result<()> {
    match data {
        _ if is_route("POST", "/register", PREFIX, &data) => register(data).await,
        _ if is_route("POST", "/login", PREFIX, &data) => login(data).await,
        _ if is_route("POST", "/me", PREFIX, &data) => me(data).await,
        _ => http_helper::not_found(data.stream).await
    }
}

async fn register(data: RequestData) -> tokio::io::Result<()> {
    let account_data: RegisterDTO = match parse_body(&data.buffer) {
        Ok(data) => data,
        Err(_) => return invalid(data.stream).await,
    };

    if !is_sha256_hash(&account_data.password) {
        return invalid(data.stream).await;
    }

    insert_account(account_data.name, account_data.password).await;

    let response_body = "Account created successfully!".to_string();
    let response = format!(
        "HTTP/1.1 201 CREATED\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    finish_request(data.stream, &*response).await
}

async fn login(data: RequestData) -> tokio::io::Result<()> {
    let login_data: LoginDTO = match parse_body(&data.buffer) {
        Ok(data) => data,
        Err(_) => return invalid(data.stream).await,
    };

    match match_and_return_account(&login_data.name, &login_data.password) {
        Some(account) => {
            let session = create_session(account.clone().id);
            let session_dto = SessionTokenDTO {
                id: account.id,
                name: account.name,
                token: session.token,
            };

            let response_body = serde_json::to_string(&session_dto)?;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            );

            finish_request(data.stream, &response).await
        },
        None => unauthenticated(data.stream).await,
    }
}

async fn me(data: RequestData) -> tokio::io::Result<()> {
    let session_data: MeDTO = match parse_body(&data.buffer) {
        Ok(data) => data,
        Err(_) => return invalid(data.stream).await,
    };

    match match_and_return_session(session_data.id, session_data.token) {
        Some(session) => {
            let account = get_account_by_id(session.clone().id).await;
            if account.is_none() {
                return unauthenticated(data.stream).await;
            }

            let session = create_session(session.clone().id);
            let session_dto = SessionTokenDTO {
                id: session.id,
                name: account.unwrap().name,
                token: session.token,
            };

            let response_body = serde_json::to_string(&session_dto)?;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            );

            finish_request(data.stream, &response).await
        }
        None => unauthenticated(data.stream).await,
    }
}
