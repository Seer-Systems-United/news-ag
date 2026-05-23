pub fn absolute_url(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        path.to_string()
    } else {
        format!("{}{}", super::config::BASE_URL, path)
    }
}

pub fn parse_date(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|date| date.with_timezone(&chrono::Utc))
        .ok()
}
