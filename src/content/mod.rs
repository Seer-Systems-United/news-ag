use std::{fmt, sync::OnceLock, time::Duration};

use scraper::{ElementRef, Html, Selector};

use crate::models::{Article, ArticleContentSource};

const USER_AGENT: &str = "Mozilla/5.0 (compatible; news-sources/0.1)";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MIN_DOCUMENT_WORDS: usize = 10;

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
        ArticleContentSource::WebPage => text_from_web_page(article.url()),
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
        ArticleContentSource::WebPage => text_from_web_page(article.url()).await,
    }
}

#[cfg(not(feature = "async"))]
fn fetch(url: &str) -> Result<String, ContentError> {
    static CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

    CLIENT
        .get_or_init(|| {
            reqwest::blocking::Client::builder()
                .timeout(REQUEST_TIMEOUT)
                .build()
                .unwrap()
        })
        .get(url)
        .header(reqwest::header::ACCEPT_ENCODING, "identity")
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()?
        .error_for_status()?
        .text()
        .map_err(ContentError::from)
}

#[cfg(feature = "async")]
async fn fetch(url: &str) -> Result<String, ContentError> {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    let request = CLIENT
        .get_or_init(|| {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = builder.timeout(REQUEST_TIMEOUT);

            builder.build().unwrap()
        })
        .get(url)
        .header(reqwest::header::ACCEPT_ENCODING, "identity")
        .header(reqwest::header::USER_AGENT, USER_AGENT);

    #[cfg(target_arch = "wasm32")]
    let request = request.timeout(REQUEST_TIMEOUT);

    request
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
        .map_err(ContentError::from)
}

#[cfg(not(feature = "async"))]
fn text_from_web_page(url: &str) -> Result<String, ContentError> {
    let result = fetch(url).and_then(|body| text_from_document(&body));

    match result {
        Ok(content) => Ok(content),
        Err(primary_error) => text_from_amp_page(url).or(Err(primary_error)),
    }
}

#[cfg(feature = "async")]
async fn text_from_web_page(url: &str) -> Result<String, ContentError> {
    let result = match fetch(url).await {
        Ok(body) => text_from_document(&body),
        Err(error) => Err(error),
    };

    match result {
        Ok(content) => Ok(content),
        Err(primary_error) => text_from_amp_page(url).await.or(Err(primary_error)),
    }
}

#[cfg(not(feature = "async"))]
fn text_from_amp_page(url: &str) -> Result<String, ContentError> {
    let url = amp_url(url).ok_or(ContentError::EmptyContent)?;
    let body = fetch(url.as_str())?;

    text_from_amp_document(&body)
}

#[cfg(feature = "async")]
async fn text_from_amp_page(url: &str) -> Result<String, ContentError> {
    let url = amp_url(url).ok_or(ContentError::EmptyContent)?;
    let body = fetch(url.as_str()).await?;

    text_from_amp_document(&body)
}

fn amp_url(url: &str) -> Option<reqwest::Url> {
    let mut url = reqwest::Url::parse(url).ok()?;
    url.query_pairs_mut().append_pair("output", "amp");
    Some(url)
}

fn text_from_amp_document(body: &str) -> Result<String, ContentError> {
    let document = Html::parse_document(body);

    json_ld_article_body(&document)
        .filter(|content| has_enough_words(content))
        .ok_or(ContentError::EmptyContent)
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
    let mut fallback = None;

    if let Some(content) = json_ld_article_body(&document) {
        if has_enough_words(&content) {
            return Ok(content);
        }

        fallback = Some(content);
    }

    if let Some(content) = embedded_json_article_body(&document) {
        return Ok(content);
    }

    for selector in [
        "[itemprop=\"articleBody\"]",
        "[data-sophi-feature=\"article body\"]",
        ".article-body",
        ".article-content",
        ".entry-content",
        "article",
        "main",
        "body",
    ] {
        let selector = Selector::parse(selector).unwrap();

        for content in document
            .select(&selector)
            .filter_map(text_from_content_element)
        {
            if has_enough_words(&content) {
                return Ok(content);
            }

            fallback.get_or_insert(content);
        }
    }

    fallback.ok_or(ContentError::EmptyContent)
}

fn embedded_json_article_body(document: &Html) -> Option<String> {
    let scripts = Selector::parse("script:not([type=\"application/ld+json\"])").unwrap();

    document.select(&scripts).find_map(|script| {
        let body = script.text().collect::<String>();

        body.match_indices(" = {").find_map(|(index, _)| {
            let value = serde_json::Deserializer::from_str(&body[index + 3..])
                .into_iter::<serde_json::Value>()
                .next()?
                .ok()?;

            find_embedded_body(&value)
        })
    })
}

fn find_embedded_body(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Object(object) => {
            for key in ["articleBody", "body"] {
                if let Some(content) = object
                    .get(key)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|body| text_from_fragment(body).ok())
                    .filter(|content| has_enough_words(content))
                {
                    return Some(content);
                }
            }

            object.values().find_map(find_embedded_body)
        }
        serde_json::Value::Array(values) => values.iter().find_map(find_embedded_body),
        _ => None,
    }
}

fn text_from_fragment(body: &str) -> Result<String, ContentError> {
    let mut content = text_from_fragment_once(body).ok_or(ContentError::EmptyContent)?;

    for _ in 0..2 {
        if !looks_like_html(&content) {
            break;
        }

        let Some(decoded) = text_from_fragment_once(&content) else {
            break;
        };

        if decoded == content {
            break;
        }

        content = decoded;
    }

    Ok(content)
}

fn text_from_fragment_once(body: &str) -> Option<String> {
    let fragment = Html::parse_fragment(body);
    text_from_content_element(fragment.root_element())
}

fn looks_like_html(content: &str) -> bool {
    content
        .char_indices()
        .filter(|(_, character)| *character == '<')
        .filter_map(|(index, _)| content[index + 1..].chars().next())
        .any(|character| character == '/' || character == '!' || character.is_ascii_alphabetic())
}

fn text_from_content_element(element: ElementRef<'_>) -> Option<String> {
    normalize(visible_text(element))
}

fn visible_text(element: ElementRef<'_>) -> impl Iterator<Item = &str> {
    element
        .descendants()
        .filter(|node| {
            !node.ancestors().any(|ancestor| {
                ancestor.value().as_element().is_some_and(|element| {
                    matches!(element.name(), "script" | "style" | "noscript")
                })
            })
        })
        .filter_map(|node| node.value().as_text())
        .map(|text| &**text)
}

fn json_ld_article_body(document: &Html) -> Option<String> {
    let scripts = Selector::parse("script[type=\"application/ld+json\"]").unwrap();

    document.select(&scripts).find_map(|script| {
        let body = script.text().collect::<String>();
        let value = serde_json::from_str::<serde_json::Value>(&body).ok()?;
        find_article_body(&value).and_then(|body| text_from_fragment(body).ok())
    })
}

fn has_enough_words(content: &str) -> bool {
    content.split_whitespace().count() > MIN_DOCUMENT_WORDS
}

pub(crate) fn has_meaningful_fragment(content: &str) -> bool {
    text_from_fragment(content).is_ok_and(|content| has_enough_words(&content))
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
    let mut text = String::new();

    for value in values {
        if should_separate_text_nodes(&text, value) {
            text.push(' ');
        }

        text.push_str(value);
    }

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

fn should_separate_text_nodes(previous: &str, next: &str) -> bool {
    let Some(previous) = previous.chars().next_back() else {
        return false;
    };
    let Some(next) = next.chars().next() else {
        return false;
    };

    !previous.is_whitespace()
        && !next.is_whitespace()
        && !matches!(previous, '(' | '[' | '{')
        && !matches!(next, '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}')
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

        assert_eq!(content, "First paragraph. Second paragraph.");
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
    fn strips_markup_from_json_ld_article_body() {
        let content = super::text_from_document(
            r#"
            <html>
                <body>
                    <script type="application/ld+json">
                        { "articleBody": "<p>Structured <strong>article</strong> body.</p>" }
                    </script>
                </body>
            </html>
            "#,
        )
        .unwrap();

        assert_eq!(content, "Structured article body.");
    }

    #[test]
    fn strips_entity_escaped_markup_from_json_ld_article_body() {
        let content = super::text_from_document(
            r#"
            <html>
                <body>
                    <script type="application/ld+json">
                        { "articleBody": "&lt;p&gt;Structured &lt;strong&gt;article&lt;/strong&gt; body.&lt;/p&gt;" }
                    </script>
                </body>
            </html>
            "#,
        )
        .unwrap();

        assert_eq!(content, "Structured article body.");
    }

    #[test]
    fn extracts_embedded_json_article_body() {
        let content = super::text_from_document(
            r#"
            <html>
                <body>
                    <script>
                        pgStoryZeroJSON = {
                            "articles": [
                                { "body": "<p>Embedded article body with enough words to satisfy the document threshold.</p>" }
                            ]
                        };
                    </script>
                </body>
            </html>
            "#,
        )
        .unwrap();

        assert_eq!(
            content,
            "Embedded article body with enough words to satisfy the document threshold."
        );
    }

    #[test]
    fn extracts_semantic_article_body_container() {
        let content = super::text_from_document(
            r#"
            <html>
                <body>
                    <article data-sophi-feature="article body">
                        <p>Semantic article body with enough words to satisfy the document threshold.</p>
                    </article>
                </body>
            </html>
            "#,
        )
        .unwrap();

        assert_eq!(
            content,
            "Semantic article body with enough words to satisfy the document threshold."
        );
    }

    #[test]
    fn removes_literal_newline_and_tab_sequences() {
        let content = super::text_from_fragment(r#"<p>First line\n\nSecond\tline.</p>"#).unwrap();
        assert_eq!(content, "First line Second line.");

        // Some feeds double-escape the content.
        let content = super::text_from_fragment(r#"<p>First line\\nSecond\\tline.</p>"#).unwrap();
        assert_eq!(content, "First line Second line.");
    }

    #[test]
    fn ignores_non_visible_fragment_text() {
        let content = super::text_from_fragment(
            r#"<article><script>const template = "<p>Hidden text.</p>";</script><p>Visible text.</p></article>"#,
        )
        .unwrap();

        assert_eq!(content, "Visible text.");
    }

    #[test]
    fn preserves_text_outside_block_elements() {
        let content =
            super::text_from_fragment("<p>Publication metadata.</p>Trailing summary.").unwrap();

        assert_eq!(content, "Publication metadata. Trailing summary.");
    }

    #[test]
    fn does_not_add_spaces_before_inline_punctuation() {
        let content =
            super::text_from_fragment("<p>A <strong>styled phrase</strong>.</p>").unwrap();

        assert_eq!(content, "A styled phrase.");
    }

    fn inline_article() -> crate::models::Article {
        crate::models::Article::new(
            "Example".to_string(),
            "https://example.com/article".to_string(),
            None,
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
