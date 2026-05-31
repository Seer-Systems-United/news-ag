use quick_xml::{
    NsReader,
    escape::unescape,
    events::{BytesCData, BytesText, Event},
    name::{Namespace, ResolveResult},
};

use crate::models::Article;

const SITEMAP_NAMESPACE: Namespace<'static> =
    Namespace(b"http://www.sitemaps.org/schemas/sitemap/0.9");
const NEWS_NAMESPACE: Namespace<'static> =
    Namespace(b"http://www.google.com/schemas/sitemap-news/0.9");

pub fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    parse_body(&fetch_body(url))
}

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

fn parse_body(body: &str) -> Vec<Article> {
    let mut reader = NsReader::from_str(body);
    reader.config_mut().trim_text(true);

    let mut articles = Vec::new();
    let mut current = None;
    let mut field = None;

    loop {
        match reader.read_resolved_event() {
            Ok((namespace, Event::Start(event))) => match event.local_name().as_ref() {
                b"url" if is_sitemap_namespace(&namespace) => {
                    current = Some(SitemapArticle::default())
                }
                b"loc" if is_sitemap_namespace(&namespace) => field = Some(Field::Url),
                b"title" if namespace == ResolveResult::Bound(NEWS_NAMESPACE) => {
                    field = Some(Field::Title)
                }
                b"publication_date" if namespace == ResolveResult::Bound(NEWS_NAMESPACE) => {
                    field = Some(Field::PublishedAt)
                }
                _ => {}
            },
            Ok((_, Event::Text(text))) => {
                if let Some(value) = decode_text(&text) {
                    append(&mut current, field, &value);
                }
            }
            Ok((_, Event::CData(text))) => {
                if let Some(value) = decode_cdata(&text) {
                    append(&mut current, field, &value);
                }
            }
            Ok((namespace, Event::End(event))) => match event.local_name().as_ref() {
                b"url" if is_sitemap_namespace(&namespace) => {
                    if let Some(article) = current.take().and_then(SitemapArticle::into_article) {
                        articles.push(article);
                    }
                }
                b"loc" if is_sitemap_namespace(&namespace) => field = None,
                b"title" | b"publication_date"
                    if namespace == ResolveResult::Bound(NEWS_NAMESPACE) =>
                {
                    field = None
                }
                _ => {}
            },
            Ok((_, Event::Eof)) | Err(_) => break,
            _ => {}
        }
    }

    articles
}

fn is_sitemap_namespace(namespace: &ResolveResult<'_>) -> bool {
    matches!(namespace, ResolveResult::Unbound)
        || matches!(
            namespace,
            ResolveResult::Bound(namespace) if *namespace == SITEMAP_NAMESPACE
        )
}

#[derive(Clone, Copy)]
enum Field {
    Url,
    Title,
    PublishedAt,
}

#[derive(Default)]
struct SitemapArticle {
    url: String,
    title: String,
    published_at: String,
}

impl SitemapArticle {
    fn into_article(self) -> Option<Article> {
        let title = clean_text(&self.title)?;
        let url = clean_text(&self.url)?;
        let published_at = chrono::DateTime::parse_from_rfc3339(self.published_at.trim())
            .map(|date| date.with_timezone(&chrono::Utc))
            .ok();

        Some(Article {
            title,
            url,
            authors: None,
            published_at,
        })
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
        None => {}
    }
}

fn decode_text(text: &BytesText<'_>) -> Option<String> {
    text.xml10_content()
        .ok()
        .and_then(|value| unescape(&value).ok().map(|value| value.into_owned()))
}

fn decode_cdata(text: &BytesCData<'_>) -> Option<String> {
    text.xml10_content().ok().map(|value| value.into_owned())
}

fn clean_text(value: &str) -> Option<String> {
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");

    if value.is_empty() { None } else { Some(value) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_google_news_sitemap_entries() {
        let articles = super::parse_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
                    xmlns:article="http://www.google.com/schemas/sitemap-news/0.9">
                <url>
                    <loc>https://example.com/article</loc>
                    <article:news>
                        <article:publication_date>2026-05-31T18:41:18Z</article:publication_date>
                        <article:title><![CDATA[Example & headline]]></article:title>
                    </article:news>
                </url>
            </urlset>"#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Example & headline");
        assert_eq!(articles[0].url, "https://example.com/article");
        assert!(articles[0].published_at.is_some());
    }
}
