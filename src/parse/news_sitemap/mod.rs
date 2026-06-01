use quick_xml::{
    Reader,
    escape::unescape,
    events::{BytesCData, BytesText, Event},
};

use crate::models::Article;

#[cfg(not(feature = "async"))]
pub fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    parse_body(&fetch_body(url))
}

#[cfg(feature = "async")]
pub async fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    parse_body(&fetch_body(url).await)
}

#[cfg(not(feature = "async"))]
fn fetch_body(url: &reqwest::Url) -> String {
    reqwest::blocking::Client::builder()
        .build()
        .unwrap()
        .get(url.as_str())
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (compatible; news-sources/0.1)",
        )
        .send()
        .unwrap()
        .text()
        .unwrap()
}

#[cfg(feature = "async")]
async fn fetch_body(url: &reqwest::Url) -> String {
    reqwest::Client::builder()
        .build()
        .unwrap()
        .get(url.as_str())
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (compatible; news-sources/0.1)",
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub fn parse_body(body: &str) -> Vec<Article> {
    let mut reader = Reader::from_str(body);
    reader.config_mut().trim_text(true);

    let mut articles = Vec::new();
    let mut current = None;
    let mut field = None;
    let mut in_news = false;
    let mut in_image = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) => match event.local_name().as_ref() {
                b"url" => current = Some(SitemapArticle::default()),
                b"loc"
                    if in_image
                        && current
                            .as_ref()
                            .is_some_and(|article| article.thumbnail_url.is_empty()) =>
                {
                    field = Some(Field::ThumbnailUrl);
                }
                b"loc"
                    if current
                        .as_ref()
                        .is_some_and(|article| article.url.is_empty()) =>
                {
                    field = Some(Field::Url);
                }
                b"news" => in_news = true,
                b"image" => in_image = true,
                b"title" if in_news => field = Some(Field::Title),
                b"publication_date" if in_news => field = Some(Field::PublishedAt),
                _ => {}
            },
            Ok(Event::Text(text)) => {
                if field.is_some() {
                    append_text(&mut current, field, &text);
                }
            }
            Ok(Event::CData(text)) => {
                if field.is_some() {
                    append_cdata(&mut current, field, &text);
                }
            }
            Ok(Event::End(event)) => match event.local_name().as_ref() {
                b"url" => {
                    if let Some(article) = current.take().and_then(SitemapArticle::into_article) {
                        articles.push(article);
                    }
                }
                b"loc" | b"title" | b"publication_date" => field = None,
                b"news" => in_news = false,
                b"image" => in_image = false,
                _ => {}
            },
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }

    articles
}

#[derive(Clone, Copy)]
enum Field {
    Url,
    Title,
    PublishedAt,
    ThumbnailUrl,
}

#[derive(Default)]
struct SitemapArticle {
    url: String,
    title: String,
    published_at: String,
    thumbnail_url: String,
}

impl SitemapArticle {
    fn into_article(self) -> Option<Article> {
        let title = clean_text(self.title)?;
        let url = clean_text(self.url)?;
        let published_at = chrono::DateTime::parse_from_rfc3339(self.published_at.trim())
            .map(|date| date.with_timezone(&chrono::Utc))
            .ok();

        Some(Article::new(
            title,
            url,
            None,
            published_at,
            clean_text(self.thumbnail_url),
        ))
    }
}

fn append(article: &mut Option<SitemapArticle>, field: Option<Field>, value: &str) {
    let Some(article) = article else {
        return;
    };

    match field {
        Some(Field::Url) => article.url.push_str(value),
        Some(Field::Title) => article.title.push_str(value),
        Some(Field::PublishedAt) => article.published_at.push_str(value),
        Some(Field::ThumbnailUrl) => article.thumbnail_url.push_str(value),
        None => {}
    }
}

fn append_text(article: &mut Option<SitemapArticle>, field: Option<Field>, text: &BytesText<'_>) {
    let Ok(decoded) = text.xml10_content() else {
        return;
    };
    let Ok(unescaped) = unescape(&decoded) else {
        return;
    };

    append(article, field, &unescaped);
}

fn append_cdata(article: &mut Option<SitemapArticle>, field: Option<Field>, text: &BytesCData<'_>) {
    if let Ok(value) = text.xml10_content() {
        append(article, field, &value);
    }
}

fn clean_text(value: String) -> Option<String> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return None;
    }

    if trimmed.len() == value.len()
        && !value.contains("  ")
        && !value
            .chars()
            .any(|character| character.is_whitespace() && character != ' ')
    {
        return Some(value);
    }

    let mut normalized = String::with_capacity(trimmed.len());
    for word in trimmed.split_whitespace() {
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        normalized.push_str(word);
    }

    Some(normalized)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_google_news_sitemap_entries() {
        let articles = super::parse_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
                    xmlns:article="http://www.google.com/schemas/sitemap-news/0.9"
                    xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
                <url>
                    <loc>https://example.com/article</loc>
                    <article:news>
                        <article:publication_date>2026-05-31T18:41:18Z</article:publication_date>
                        <article:title><![CDATA[Example & headline]]></article:title>
                    </article:news>
                    <image:image>
                        <image:loc>https://example.com/image.jpg</image:loc>
                    </image:image>
                    <image:image>
                        <image:loc>https://example.com/second-image.jpg</image:loc>
                    </image:image>
                </url>
            </urlset>"#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Example & headline");
        assert_eq!(articles[0].url, "https://example.com/article");
        assert!(articles[0].published_at.is_some());
        assert_eq!(
            articles[0].thumbnail_url(),
            Some("https://example.com/image.jpg")
        );
    }
}
