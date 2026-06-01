#[derive(Debug, Clone)]
pub(crate) enum ArticleContentSource {
    InlineHtml(String),
    WordpressRest(reqwest::Url),
    WebPage,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub authors: Option<Vec<String>>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub thumbnail_url: Option<String>,
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
