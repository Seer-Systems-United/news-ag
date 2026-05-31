#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub authors: Option<Vec<String>>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn authors(&self) -> Option<&[String]> {
        self.authors.as_deref()
    }

    pub fn published_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.published_at
    }
}
