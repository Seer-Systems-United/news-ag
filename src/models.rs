#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub authors: Option<Vec<String>>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}
