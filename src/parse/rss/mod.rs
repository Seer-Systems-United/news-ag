use std::{io::Cursor, sync::OnceLock, time::Duration};

use crate::models::Article;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

#[cfg(not(feature = "async"))]
pub fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    let articles = fetch_body(url, true)
        .map(|body| parse_body(&body))
        .unwrap_or_default();

    if articles.is_empty() {
        fetch_body(url, false)
            .map(|body| parse_body(&body))
            .unwrap_or_default()
    } else {
        articles
    }
}

#[cfg(feature = "async")]
pub async fn parse(url: &reqwest::Url, _rules: &[crate::parse::rule::Rule]) -> Vec<Article> {
    let articles = fetch_body(url, true)
        .await
        .map(|body| parse_body(&body))
        .unwrap_or_default();

    if articles.is_empty() {
        fetch_body(url, false)
            .await
            .map(|body| parse_body(&body))
            .unwrap_or_default()
    } else {
        articles
    }
}

#[cfg(not(feature = "async"))]
fn fetch_body(url: &reqwest::Url, use_user_agent: bool) -> Option<String> {
    static CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

    let mut request = CLIENT
        .get_or_init(|| {
            reqwest::blocking::Client::builder()
                .timeout(REQUEST_TIMEOUT)
                .build()
                .unwrap()
        })
        .get(url.as_str());

    request = request.header(
        reqwest::header::USER_AGENT,
        if use_user_agent {
            "Mozilla/5.0 (compatible; news-sources/0.1)"
        } else {
            "curl/8.19.0"
        },
    );

    request.send().ok()?.error_for_status().ok()?.text().ok()
}

#[cfg(feature = "async")]
async fn fetch_body(url: &reqwest::Url, use_user_agent: bool) -> Option<String> {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    let mut request = CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .timeout(REQUEST_TIMEOUT)
                .build()
                .unwrap()
        })
        .get(url.as_str());

    request = request.header(
        reqwest::header::USER_AGENT,
        if use_user_agent {
            "Mozilla/5.0 (compatible; news-sources/0.1)"
        } else {
            "curl/8.19.0"
        },
    );

    request
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .text()
        .await
        .ok()
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
        rss_thumbnail_url(item),
    );

    Some(with_inline_content(article, item.content()))
}

fn article_from_atom_entry(entry: &atom_syndication::Entry) -> Option<Article> {
    let article = Article::new(
        text(Some(entry.title().as_str()))?,
        atom_link(entry)?,
        atom_authors(entry),
        atom_published_at(entry),
        atom_thumbnail_url(entry),
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

fn rss_thumbnail_url(item: &rss::Item) -> Option<String> {
    rss_extension_thumbnail_url(item.extensions()).or_else(|| {
        item.enclosure()
            .filter(|enclosure| enclosure.mime_type().starts_with("image/"))
            .and_then(|enclosure| text(Some(enclosure.url())))
    })
}

fn rss_extension_thumbnail_url(extensions: &rss::extension::ExtensionMap) -> Option<String> {
    extensions
        .values()
        .find_map(rss_extension_map_thumbnail_url)
}

fn rss_extension_map_thumbnail_url(
    extensions: &std::collections::BTreeMap<String, Vec<rss::extension::Extension>>,
) -> Option<String> {
    extensions
        .get("thumbnail")
        .and_then(|extensions| extensions.iter().find_map(rss_extension_url))
        .or_else(|| {
            extensions.get("content").and_then(|extensions| {
                extensions
                    .iter()
                    .filter(|extension| extension_is_image(extension.attrs()))
                    .find_map(rss_extension_url)
            })
        })
        .or_else(|| {
            extensions
                .values()
                .flatten()
                .find_map(|extension| rss_extension_map_thumbnail_url(extension.children()))
        })
}

fn rss_extension_url(extension: &rss::extension::Extension) -> Option<String> {
    extension
        .attrs()
        .get("url")
        .or_else(|| extension.attrs().get("href"))
        .map(String::as_str)
        .or_else(|| extension.value())
        .and_then(|url| text(Some(url)))
}

fn atom_link(entry: &atom_syndication::Entry) -> Option<String> {
    entry
        .links()
        .iter()
        .find(|link| link.rel() == "alternate")
        .or_else(|| entry.links().first())
        .and_then(|link| text(Some(link.href())))
}

fn atom_thumbnail_url(entry: &atom_syndication::Entry) -> Option<String> {
    atom_extension_thumbnail_url(entry.extensions()).or_else(|| {
        entry
            .links()
            .iter()
            .find(|link| {
                link.rel() == "enclosure"
                    && link
                        .mime_type()
                        .is_some_and(|mime| mime.starts_with("image/"))
            })
            .and_then(|link| text(Some(link.href())))
    })
}

fn atom_extension_thumbnail_url(
    extensions: &atom_syndication::extension::ExtensionMap,
) -> Option<String> {
    extensions
        .values()
        .find_map(atom_extension_map_thumbnail_url)
}

fn atom_extension_map_thumbnail_url(
    extensions: &std::collections::BTreeMap<String, Vec<atom_syndication::extension::Extension>>,
) -> Option<String> {
    extensions
        .get("thumbnail")
        .and_then(|extensions| extensions.iter().find_map(atom_extension_url))
        .or_else(|| {
            extensions.get("content").and_then(|extensions| {
                extensions
                    .iter()
                    .filter(|extension| extension_is_image(extension.attrs()))
                    .find_map(atom_extension_url)
            })
        })
        .or_else(|| {
            extensions
                .values()
                .flatten()
                .find_map(|extension| atom_extension_map_thumbnail_url(extension.children()))
        })
}

fn atom_extension_url(extension: &atom_syndication::extension::Extension) -> Option<String> {
    extension
        .attrs()
        .get("url")
        .or_else(|| extension.attrs().get("href"))
        .map(String::as_str)
        .or_else(|| extension.value())
        .and_then(|url| text(Some(url)))
}

fn extension_is_image(attrs: &std::collections::BTreeMap<String, String>) -> bool {
    attrs.get("medium").is_some_and(|medium| medium == "image")
        || attrs
            .get("type")
            .is_some_and(|mime_type| mime_type.starts_with("image/"))
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
    #[cfg(not(feature = "async"))]
    #[test]
    fn returns_empty_articles_when_feed_request_fails() {
        let url = "http://127.0.0.1:0/feed".parse().unwrap();

        assert!(super::parse(&url, &[]).is_empty());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn returns_empty_articles_when_feed_request_fails() {
        let url = "http://127.0.0.1:0/feed".parse().unwrap();

        assert!(super::parse(&url, &[]).await.is_empty());
    }

    #[test]
    fn parses_rss_items() {
        let articles = super::parse_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0"
                 xmlns:dc="http://purl.org/dc/elements/1.1/"
                 xmlns:media="http://search.yahoo.com/mrss/">
                <channel>
                    <title>Example Feed</title>
                    <link>https://example.com</link>
                    <description>Example</description>
                    <item>
                        <title><![CDATA[Example &amp; headline]]></title>
                        <link>https://example.com/article</link>
                        <dc:creator><![CDATA[Reporter]]></dc:creator>
                        <pubDate>Sat, 23 May 2026 06:00:00 GMT</pubDate>
                        <media:thumbnail url="https://example.com/image.jpg"/>
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
        assert_eq!(
            articles[0].thumbnail_url(),
            Some("https://example.com/image.jpg")
        );
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
                    <link href="https://example.com/image.jpg" rel="enclosure" type="image/jpeg"/>
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
        assert_eq!(
            articles[0].thumbnail_url(),
            Some("https://example.com/image.jpg")
        );
    }

    #[test]
    fn decodes_known_entities_and_preserves_unknown_entities() {
        assert_eq!(
            super::decode_entities("A &amp; B &nbsp; C &#39;"),
            "A & B &nbsp; C '"
        );
    }
}
