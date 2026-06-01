#[derive(Debug, Clone)]
pub(crate) enum ArticleContentSource {
    InlineHtml(String),
    WordpressRest(reqwest::Url),
    WebPage,
}

impl Default for ArticleContentSource {
    fn default() -> Self {
        Self::WebPage
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Article {
    pub title: String,
    pub url: String,
    pub authors: Option<Vec<String>>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub thumbnail_url: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip, default))]
    content_source: ArticleContentSource,
}

impl Article {
    pub fn new(
        title: String,
        url: String,
        authors: Option<Vec<String>>,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
        thumbnail_url: Option<String>,
    ) -> Self {
        Self {
            title,
            url,
            authors,
            published_at,
            content_source: ArticleContentSource::WebPage,
            thumbnail_url,
        }
    }

    pub(crate) fn with_inline_content(mut self, content: &str) -> Self {
        self.content_source = ArticleContentSource::InlineHtml(content.to_string());
        self
    }

    pub(crate) fn with_wordpress_content_url(mut self, mut url: reqwest::Url) -> Self {
        url.query_pairs_mut()
            .append_pair("_fields", "content.rendered");
        self.content_source = ArticleContentSource::WordpressRest(url);
        self
    }

    pub(crate) fn content_source(&self) -> &ArticleContentSource {
        &self.content_source
    }

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

    pub fn thumbnail_url(&self) -> Option<&str> {
        self.thumbnail_url.as_deref()
    }

    #[cfg(not(feature = "async"))]
    pub fn get_content(&self) -> Result<String, crate::content::ContentError> {
        crate::content::get(self)
    }

    #[cfg(feature = "async")]
    pub async fn get_content(&self) -> Result<String, crate::content::ContentError> {
        crate::content::get(self).await
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    #[test]
    fn article_round_trips_through_json() {
        let published_at = chrono::DateTime::parse_from_rfc3339("2026-06-01T18:30:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let article = super::Article::new(
            "Example headline".to_string(),
            "https://example.com/article".to_string(),
            Some(vec!["Reporter".to_string()]),
            Some(published_at),
            Some("https://example.com/image.jpg".to_string()),
        )
        .with_inline_content("<p>Runtime-only content source.</p>");

        let json = serde_json::to_string(&article).unwrap();
        let decoded: super::Article = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.title(), article.title());
        assert_eq!(decoded.url(), article.url());
        assert_eq!(decoded.authors(), article.authors());
        assert_eq!(decoded.published_at(), article.published_at());
        assert_eq!(decoded.thumbnail_url(), article.thumbnail_url());
        assert!(matches!(
            decoded.content_source(),
            super::ArticleContentSource::WebPage
        ));
        assert!(!json.contains("content_source"));
    }
}
