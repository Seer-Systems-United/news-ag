use std::io::Cursor;

use crate::models::Article;

pub fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    let body = reqwest::blocking::get(url.as_str())
        .unwrap()
        .text()
        .unwrap();

    parse_body(&body)
}

fn parse_body(body: &str) -> Vec<Article> {
    let Ok(channel) = rss::Channel::read_from(Cursor::new(body.as_bytes())) else {
        return Vec::new();
    };

    channel
        .items()
        .iter()
        .filter_map(article_from_item)
        .collect()
}

fn article_from_item(item: &rss::Item) -> Option<Article> {
    Some(Article {
        title: text(item.title())?,
        url: text(item.link())?,
        authors: authors(item),
        published_at: published_at(item),
    })
}

fn authors(item: &rss::Item) -> Option<Vec<String>> {
    let authors = item
        .dublin_core_ext()
        .map(|extension| extension.creators().to_vec())
        .filter(|authors| !authors.is_empty())
        .or_else(|| item.author().map(|author| vec![author.to_string()]))?;

    let authors = authors
        .into_iter()
        .map(|author| decode_entities(author.trim()))
        .filter(|author| !author.is_empty())
        .collect::<Vec<_>>();

    if authors.is_empty() {
        None
    } else {
        Some(authors)
    }
}

fn published_at(item: &rss::Item) -> Option<chrono::DateTime<chrono::Utc>> {
    item.pub_date()
        .or_else(|| {
            item.dublin_core_ext()
                .and_then(|extension| extension.dates().first().map(String::as_str))
        })
        .and_then(parse_date)
}

fn parse_date(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc2822(value)
        .or_else(|_| chrono::DateTime::parse_from_rfc3339(value))
        .map(|date| date.with_timezone(&chrono::Utc))
        .ok()
}

fn text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(decode_entities)
}

fn decode_entities(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_rss_items() {
        let articles = super::parse_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0" xmlns:dc="http://purl.org/dc/elements/1.1/">
                <channel>
                    <title>Example Feed</title>
                    <link>https://example.com</link>
                    <description>Example</description>
                    <item>
                        <title><![CDATA[Example &amp; headline]]></title>
                        <link>https://example.com/article</link>
                        <dc:creator><![CDATA[Reporter]]></dc:creator>
                        <pubDate>Sat, 23 May 2026 06:00:00 GMT</pubDate>
                    </item>
                </channel>
            </rss>"#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Example & headline");
        assert_eq!(articles[0].url, "https://example.com/article");
        assert_eq!(
            articles[0].authors.as_ref().unwrap(),
            &vec!["Reporter".to_string()]
        );
        assert!(articles[0].published_at.is_some());
    }
}
