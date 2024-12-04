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