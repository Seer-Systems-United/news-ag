use std::io::Cursor;

use crate::models::Article;

#[cfg(not(feature = "async"))]
pub fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    let body = fetch_body(url, true);
    let articles = parse_body(&body);

    if articles.is_empty() {
        parse_body(&fetch_body(url, false))
    } else {
        articles
    }
}

#[cfg(feature = "async")]
pub async fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    let body = fetch_body(url, true).await;
    let articles = parse_body(&body);

    if articles.is_empty() {
        parse_body(&fetch_body(url, false).await)
    } else {
        articles
    }
}

#[cfg(not(feature = "async"))]
fn fetch_body(url: &reqwest::Url, use_user_agent: bool) -> String {
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let mut request = client.get(url.as_str());

    request = request.header(
        reqwest::header::USER_AGENT,
        if use_user_agent {
            "Mozilla/5.0 (compatible; news-sources/0.1)"
        } else {
            "curl/8.19.0"
        },
    );

    request.send().unwrap().text().unwrap()
}

#[cfg(feature = "async")]
async fn fetch_body(url: &reqwest::Url, use_user_agent: bool) -> String {
    let client = reqwest::Client::builder().build().unwrap();
    let mut request = client.get(url.as_str());

    request = request.header(
        reqwest::header::USER_AGENT,
        if use_user_agent {
            "Mozilla/5.0 (compatible; news-sources/0.1)"
        } else {
            "curl/8.19.0"
        },
    );

    request.send().await.unwrap().text().await.unwrap()
}

pub fn parse_body(body: &str) -> Vec<Article> {
    if let Ok(channel) = rss::Channel::read_from(Cursor::new(body.as_bytes())) {
        return channel
            .items()
            .iter()
            .filter_map(article_from_rss_item)
            .collect();
    }

    let Ok(feed) = atom_syndication::Feed::read_from(Cursor::new(body.as_bytes())) else {
        return Vec::new();
    };

    feed.entries()
        .iter()
        .filter_map(article_from_atom_entry)
        .collect()
}

fn article_from_rss_item(item: &rss::Item) -> Option<Article> {
    let article = Article::new(
        text(item.title())?,
        text(item.link())?,
        rss_authors(item),
        rss_published_at(item),
    );

    Some(with_inline_content(article, item.content()))
}

fn article_from_atom_entry(entry: &atom_syndication::Entry) -> Option<Article> {
    let article = Article::new(
        text(Some(entry.title().as_str()))?,
        atom_link(entry)?,
        atom_authors(entry),
        atom_published_at(entry),
    );

    Some(with_inline_content(article, atom_inline_content(entry)))
}

fn with_inline_content(article: Article, content: Option<&str>) -> Article {
    if let Some(content) = content
        .map(str::trim)
        .filter(|content| crate::content::has_meaningful_fragment(content))
    {
        article.with_inline_content(content)
    } else {
        article
    }
}

fn atom_inline_content(entry: &atom_syndication::Entry) -> Option<&str> {
    let content = entry.content()?;

    match content.content_type() {
        None
        | Some("text")
        | Some("html")
        | Some("xhtml")
        | Some("text/plain")
        | Some("text/html")
        | Some("application/xhtml+xml") => content.value(),
        _ => None,
    }
}

fn rss_authors(item: &rss::Item) -> Option<Vec<String>> {
    let authors = if let Some(authors) = item
        .dublin_core_ext()
        .map(|extension| extension.creators())
        .filter(|authors| !authors.is_empty())
    {
        authors
            .iter()
            .map(|author| decode_entities(author.trim()))
            .filter(|author| !author.is_empty())
            .collect::<Vec<_>>()
    } else {
        item.author()
            .map(|author| decode_entities(author.trim()))
            .filter(|author| !author.is_empty())
            .into_iter()
            .collect::<Vec<_>>()
    };

    if authors.is_empty() {
        None
    } else {
        Some(authors)
    }
}

fn rss_published_at(item: &rss::Item) -> Option<chrono::DateTime<chrono::Utc>> {
    item.pub_date()
        .or_else(|| {
            item.dublin_core_ext()
                .and_then(|extension| extension.dates().first().map(String::as_str))
        })
        .and_then(parse_date)
}

fn atom_link(entry: &atom_syndication::Entry) -> Option<String> {
    entry
        .links()
        .iter()
        .find(|link| link.rel() == "alternate")
        .or_else(|| entry.links().first())
        .and_then(|link| text(Some(link.href())))
}

fn atom_authors(entry: &atom_syndication::Entry) -> Option<Vec<String>> {
    let authors = entry
        .authors()
        .iter()
        .map(|author| decode_entities(author.name().trim()))
        .filter(|author| !author.is_empty())
        .collect::<Vec<_>>();

    if authors.is_empty() {
        None
    } else {
        Some(authors)
    }
}

fn atom_published_at(entry: &atom_syndication::Entry) -> Option<chrono::DateTime<chrono::Utc>> {
    Some(
        entry
            .published()
            .unwrap_or_else(|| entry.updated())
            .with_timezone(&chrono::Utc),
    )
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
    let Some(_) = value.find('&') else {
        return value.to_string();
    };

    let mut decoded = String::with_capacity(value.len());
    let mut remainder = value;

    while let Some(start) = remainder.find('&') {
        decoded.push_str(&remainder[..start]);

        let Some(end) = remainder[start + 1..].find(';') else {
            decoded.push_str(&remainder[start..]);
            return decoded;
        };
        let end = start + end + 1;
        let entity = &remainder[start + 1..end];

        decoded.push_str(match entity {
            "amp" => "&",
            "lt" => "<",
            "gt" => ">",
            "quot" => "\"",
            "apos" | "#39" => "'",
            _ => &remainder[start..=end],
        });

        remainder = &remainder[end + 1..];
    }

    decoded.push_str(remainder);
    decoded
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
                        <content:encoded xmlns:content="http://purl.org/rss/1.0/modules/content/"><![CDATA[<p>Full article content with enough words to be useful to library consumers.</p>]]></content:encoded>
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
        assert!(matches!(
            articles[0].content_source(),
            crate::models::ArticleContentSource::InlineHtml(content)
                if content == "<p>Full article content with enough words to be useful to library consumers.</p>"
        ));
    }

    #[test]
    fn ignores_short_inline_content() {
        let article = crate::models::Article::new(
            "Example".to_string(),
            "https://example.com/article".to_string(),
            None,
            None,
        );
        let article = super::with_inline_content(article, Some("<p>Short summary.</p>"));

        assert!(matches!(
            article.content_source(),
            crate::models::ArticleContentSource::WebPage
        ));
    }

    #[test]
    fn parses_atom_entries() {
        let articles = super::parse_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <feed xmlns="http://www.w3.org/2005/Atom">
                <title>Example Feed</title>
                <id>https://example.com</id>
                <updated>2026-05-23T06:00:00Z</updated>
                <entry>
                    <title>Example headline</title>
                    <link href="https://example.com/article" rel="alternate"/>
                    <id>https://example.com/article</id>
                    <updated>2026-05-23T06:00:00Z</updated>
                    <author><name>Reporter</name></author>
                </entry>
            </feed>"#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Example headline");
        assert_eq!(articles[0].url, "https://example.com/article");
        assert_eq!(
            articles[0].authors.as_ref().unwrap(),
            &vec!["Reporter".to_string()]
        );
        assert!(articles[0].published_at.is_some());
    }

    #[test]
    fn decodes_known_entities_and_preserves_unknown_entities() {
        assert_eq!(
            super::decode_entities("A &amp; B &nbsp; C &#39;"),
            "A & B &nbsp; C '"
        );
    }
}
