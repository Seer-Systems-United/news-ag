use std::{fmt, sync::OnceLock};

use scraper::{ElementRef, Html, Selector};

use crate::models::{Article, ArticleContentSource};

const USER_AGENT: &str = "Mozilla/5.0 (compatible; news-sources/0.1)";

#[derive(Debug)]
pub enum ContentError {
    EmptyContent,
    InvalidWordpressResponse(serde_json::Error),
    Request(reqwest::Error),
}

impl fmt::Display for ContentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyContent => formatter.write_str("article content was empty"),
            Self::InvalidWordpressResponse(error) => {
                write!(formatter, "invalid WordPress content response: {error}")
            }
            Self::Request(error) => write!(formatter, "article content request failed: {error}"),
        }
    }
}

impl std::error::Error for ContentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EmptyContent => None,
            Self::InvalidWordpressResponse(error) => Some(error),
            Self::Request(error) => Some(error),
        }
    }
}

impl From<reqwest::Error> for ContentError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

#[cfg(not(feature = "async"))]
pub(crate) fn get(article: &Article) -> Result<String, ContentError> {
    match article.content_source() {
        ArticleContentSource::InlineHtml(content) => text_from_fragment(content),
        ArticleContentSource::WordpressRest(url) => {
            let body = fetch(url.as_str())?;
            text_from_wordpress_response(&body)
        }
        ArticleContentSource::WebPage => {
            let body = fetch(article.url())?;
            text_from_document(&body)
        }
    }
}

#[cfg(feature = "async")]
pub(crate) async fn get(article: &Article) -> Result<String, ContentError> {
    match article.content_source() {
        ArticleContentSource::InlineHtml(content) => text_from_fragment(content),
        ArticleContentSource::WordpressRest(url) => {
            let body = fetch(url.as_str()).await?;
            text_from_wordpress_response(&body)
        }
        ArticleContentSource::WebPage => {
            let body = fetch(article.url()).await?;
            text_from_document(&body)
        }
    }
}

#[cfg(not(feature = "async"))]
fn fetch(url: &str) -> Result<String, ContentError> {
    static CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

    CLIENT
        .get_or_init(|| reqwest::blocking::Client::builder().build().unwrap())
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()?
        .error_for_status()?
        .text()
        .map_err(ContentError::from)
}

#[cfg(feature = "async")]
async fn fetch(url: &str) -> Result<String, ContentError> {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    CLIENT
        .get_or_init(|| reqwest::Client::builder().build().unwrap())
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
        .map_err(ContentError::from)
}

#[derive(serde::Deserialize)]
struct WordpressPost {
    content: RenderedText,
}

#[derive(serde::Deserialize)]
struct RenderedText {
    rendered: String,
}

fn text_from_wordpress_response(body: &str) -> Result<String, ContentError> {
    let post = serde_json::from_str::<WordpressPost>(body)
        .map_err(ContentError::InvalidWordpressResponse)?;

    text_from_fragment(&post.content.rendered)
}

fn text_from_document(body: &str) -> Result<String, ContentError> {
    let document = Html::parse_document(body);

    if let Some(content) = json_ld_article_body(&document) {
        return Ok(content);
    }

    for selector in [
        "[itemprop=\"articleBody\"]",
        ".article-body",
        ".article-content",
        ".entry-content",
        "article",
        "main",
        "body",
    ] {
        let selector = Selector::parse(selector).unwrap();

        if let Some(content) = document
            .select(&selector)
            .find_map(text_from_content_element)
        {
            return Ok(content);
        }
    }

    Err(ContentError::EmptyContent)
}

fn text_from_fragment(body: &str) -> Result<String, ContentError> {
    let fragment = Html::parse_fragment(body);
    text_from_content_element(fragment.root_element()).ok_or(ContentError::EmptyContent)
}

fn text_from_content_element(element: ElementRef<'_>) -> Option<String> {
    let blocks = Selector::parse("p, li, blockquote, h1, h2, h3, h4, h5, h6").unwrap();
    let content = element
        .select(&blocks)
        .filter_map(|block| normalize(block.text()))
        .collect::<Vec<_>>()
        .join("\n\n");

    if content.is_empty() {
        normalize(element.text())
    } else {
        Some(content)
    }
}

fn json_ld_article_body(document: &Html) -> Option<String> {
    let scripts = Selector::parse("script[type=\"application/ld+json\"]").unwrap();

    document.select(&scripts).find_map(|script| {
        let body = script.text().collect::<String>();
        let value = serde_json::from_str::<serde_json::Value>(&body).ok()?;
        find_article_body(&value).and_then(|body| normalize(std::iter::once(body)))
    })
}

fn find_article_body(value: &serde_json::Value) -> Option<&str> {
    match value {
        serde_json::Value::Object(object) => object
            .get("articleBody")
            .and_then(serde_json::Value::as_str)
            .or_else(|| object.values().find_map(find_article_body)),
        serde_json::Value::Array(values) => values.iter().find_map(find_article_body),
        _ => None,
    }
}

fn normalize<'a>(values: impl Iterator<Item = &'a str>) -> Option<String> {
    let text = values.collect::<String>();
    let text = unescape_literal_whitespace(&text);

    let mut normalized = String::new();

    for word in text.split_whitespace() {
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        normalized.push_str(word);
    }

    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

/// Some sources embed literal escape sequences (e.g. `\\n`, `\\t`) in otherwise plain text.
///
/// We convert these to real whitespace so downstream normalization (`split_whitespace`) can
/// collapse them cleanly.
fn unescape_literal_whitespace(text: &str) -> String {
    // Fast path: avoid allocations in the common case.
    if !(text.contains("\\n")
        || text.contains("\\t")
        || text.contains("\\r")
        || text.contains("\\\\n")
        || text.contains("\\\\t")
        || text.contains("\\\\r"))
    {
        return text.to_string();
    }

    // Handle double-escaped sequences first (e.g. `\\n`).
    // Then handle single-escaped sequences (`\n`).
    //
    // This avoids leaving stray backslashes behind when the source has been escaped twice.
    text.replace("\\\\r\\\\n", "\n")
        .replace("\\\\n", "\n")
        .replace("\\\\t", "\t")
        .replace("\\\\r", "\r")
        // Handle CRLF first so we don't leave stray `\r` behind.
        .replace("\\r\\n", "\n")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
}

#[cfg(test)]
mod tests {
    #[test]
    fn extracts_wordpress_rendered_content_as_text() {
        let content = super::text_from_wordpress_response(
            r#"{
                "content": {
                    "rendered": "<p>First <strong>paragraph</strong>.</p><p>Second paragraph.</p>"
                }
            }"#,
        )
        .unwrap();

        assert_eq!(content, "First paragraph.\n\nSecond paragraph.");
    }

    #[test]
    fn prefers_json_ld_article_body() {
        let content = super::text_from_document(
            r#"
            <html>
                <body>
                    <article><p>Visible fallback.</p></article>
                    <script type="application/ld+json">
                        { "articleBody": "Structured article body." }
                    </script>
                </body>
            </html>
            "#,
        )
        .unwrap();

        assert_eq!(content, "Structured article body.");
    }

    #[test]
    fn removes_literal_newline_and_tab_sequences() {
        let content = super::text_from_fragment(r#"<p>First line\n\nSecond\tline.</p>"#).unwrap();
        assert_eq!(content, "First line Second line.");

        // Some feeds double-escape the content.
        let content = super::text_from_fragment(r#"<p>First line\\nSecond\\tline.</p>"#).unwrap();
        assert_eq!(content, "First line Second line.");
    }

    fn inline_article() -> crate::models::Article {
        crate::models::Article::new(
            "Example".to_string(),
            "https://example.com/article".to_string(),
            None,
            None,
        )
        .with_inline_content("<p>Inline <strong>article</strong>.</p>")
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn article_get_content_returns_inline_text() {
        assert_eq!(inline_article().get_content().unwrap(), "Inline article.");
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn article_get_content_returns_inline_text() {
        assert_eq!(
            inline_article().get_content().await.unwrap(),
            "Inline article."
        );
    }
}
