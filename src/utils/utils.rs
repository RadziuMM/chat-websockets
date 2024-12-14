use tokio::io;
use crate::entity::session::Session;
use crate::repository::session::match_and_return_session;
use crate::utils::http_helper::{parse_cookies};

pub fn extract_path_from_request(request: &str) -> Option<String> {
    let lines: Vec<&str> = request.lines().collect();
    if let Some(first_line) = lines.first() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            return Some(parts[1].to_string());
        }
    }
    None
}

pub fn authorize(buffer: &[u8; 1024]) -> io::Result<Session> {
    let cookies = parse_cookies(std::str::from_utf8(buffer).unwrap_or_default());
    let id = cookies.get("id");
    let session = cookies.get("token");
    if id.is_none() || session.is_none() {
        return Err(io::Error::new(io::ErrorKind::Other, "unauthenticated"))
    }

    match match_and_return_session(id.unwrap().clone(), session.unwrap().clone()) {
        Some(session) => Ok(session),
        None => Err(io::Error::new(io::ErrorKind::Other, "unauthenticated")),
    }
}

pub fn clear_cookies_response() -> String {
    let cookies_to_clear = vec!["id", "token", "name"];
    let mut headers = String::new();

    for cookie in cookies_to_clear {
        headers.push_str(&format!(
            "Set-Cookie: {}=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict\r\n",
            cookie
        ));
    }

    headers
}