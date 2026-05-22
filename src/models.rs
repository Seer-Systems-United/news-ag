#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub author: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}
