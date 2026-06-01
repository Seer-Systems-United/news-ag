use std::borrow::Cow;

use crate::models::Article;

#[derive(Debug, serde::Deserialize)]
struct WordpressPost {
    date: Option<String>,
    date_gmt: Option<String>,
    link: Option<String>,
    title: Option<RenderedText>,
    yoast_head_json: Option<YoastHeadJson>,
    jetpack_featured_media_url: Option<String>,
    #[serde(rename = "_embedded")]
    embedded: Option<WordpressEmbedded>,
    #[serde(rename = "_links")]
    links: Option<WordpressLinks>,
}

#[derive(Debug, serde::Deserialize)]
struct RenderedText {
    rendered: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct YoastHeadJson {
    author: Option<String>,
    og_image: Option<Vec<YoastImage>>,
}

#[derive(Debug, serde::Deserialize)]
struct YoastImage {
    url: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct WordpressEmbedded {
    #[serde(rename = "wp:featuredmedia")]
    featured_media: Option<Vec<WordpressFeaturedMedia>>,
}

#[derive(Debug, serde::Deserialize)]
struct WordpressFeaturedMedia {
    source_url: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct WordpressLinks {
    #[serde(rename = "self")]
    self_links: Option<Vec<WordpressLink>>,
}

#[derive(Debug, serde::Deserialize)]
struct WordpressLink {
    href: Option<String>,
}

pub fn parse_posts(body: &str) -> Vec<Article> {
    serde_json::from_str::<Vec<WordpressPost>>(body)
        .unwrap_or_default()
        .into_iter()
        .filter_map(article_from_post)
        .collect()
}

fn article_from_post(post: WordpressPost) -> Option<Article> {
    let WordpressPost {
        date,
        date_gmt,
        link,
        title,
        yoast_head_json,
        jetpack_featured_media_url,
        embedded,
        links,
    } = post;

    let thumbnail_url = thumbnail_url(
        yoast_head_json.as_ref(),
        jetpack_featured_media_url,
        embedded,
    );
    let article = Article::new(
        clean_text(&title?.rendered?)?,
        text(link.as_deref())?,
        authors(yoast_head_json.as_ref()),
        date_gmt
            .as_deref()
            .and_then(parse_wordpress_date)
            .or_else(|| date.as_deref().and_then(parse_wordpress_date)),
        thumbnail_url,
    );

    Some(if let Some(url) = wordpress_content_url(links) {
        article.with_wordpress_content_url(url)
    } else {
        article
    })
}

fn wordpress_content_url(links: Option<WordpressLinks>) -> Option<reqwest::Url> {
    links?
        .self_links?
        .into_iter()
        .find_map(|link| link.href?.parse().ok())
}

fn authors(yoast_head_json: Option<&YoastHeadJson>) -> Option<Vec<String>> {
    let author = clean_text(yoast_head_json?.author.as_deref()?)?;
    Some(vec![author])
}

fn thumbnail_url(
    yoast_head_json: Option<&YoastHeadJson>,
    jetpack_featured_media_url: Option<String>,
    embedded: Option<WordpressEmbedded>,
) -> Option<String> {
    jetpack_featured_media_url
        .as_deref()
        .and_then(|url| text(Some(url)))
        .or_else(|| {
            yoast_head_json?
                .og_image
                .as_ref()?
                .iter()
                .find_map(|image| text(image.url.as_deref()))
        })
        .or_else(|| {
            embedded?
                .featured_media?
                .into_iter()
                .find_map(|media| text(media.source_url.as_deref()))
        })
}

fn parse_wordpress_date(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    if value == "0000-00-00T00:00:00" {
        return None;
    }

    chrono::DateTime::parse_from_rfc3339(value)
        .map(|date| date.with_timezone(&chrono::Utc))
        .ok()
        .or_else(|| {
            chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
                .map(|date| date.and_utc())
                .ok()
        })
}

fn text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn clean_text(value: &str) -> Option<String> {
    let stripped = strip_tags(value);
    let decoded = match quick_xml::escape::unescape(&stripped) {
        Ok(Cow::Owned(decoded)) => Cow::Owned(decoded),
        Ok(Cow::Borrowed(_)) => stripped,
        Err(_) => return clean_text_with_html_parser(value),
    };

    normalize_text(decoded)
}

fn strip_tags(value: &str) -> Cow<'_, str> {
    if !value.contains('<') {
        return Cow::Borrowed(value);
    }

    let mut text = String::with_capacity(value.len());
    let mut in_tag = false;

    for character in value.chars() {
        match character {
            '<' => {
                in_tag = true;
                text.push(' ');
            }
            '>' if in_tag => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag => text.push(character),
            _ => {}
        }
    }

    Cow::Owned(text)
}

fn normalize_text(value: Cow<'_, str>) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.len() == value.len()
        && !trimmed.contains("  ")
        && !trimmed
            .chars()
            .any(|character| character.is_whitespace() && character != ' ')
    {
        return Some(value.into_owned());
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

fn clean_text_with_html_parser(value: &str) -> Option<String> {
    let fragment = scraper::Html::parse_fragment(value);
    let text = fragment.root_element().text().collect::<Vec<_>>().join(" ");
    normalize_text(Cow::Owned(text))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_wordpress_posts() {
        let articles = super::parse_posts(
            r#"[
                {
                    "date": "2026-05-28T19:08:37",
                    "date_gmt": "2026-05-29T01:08:37",
                    "link": "https://example.com/news",
                    "title": { "rendered": "Gov. Jared Polis&#8217; first <em>vetoes</em>" },
                    "yoast_head_json": {
                        "author": "Jane Reporter",
                        "og_image": [{ "url": "https://example.com/yoast-image.jpg" }]
                    },
                    "jetpack_featured_media_url": "https://example.com/featured-image.jpg",
                    "_links": {
                        "self": [
                            { "href": "https://example.com/wp-json/wp/v2/posts/42" }
                        ]
                    }
                }
            ]"#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Gov. Jared Polis\u{2019} first vetoes");
        assert_eq!(articles[0].url, "https://example.com/news");
        assert_eq!(
            articles[0].authors.as_ref().unwrap(),
            &vec!["Jane Reporter".to_string()]
        );
        assert!(articles[0].published_at.is_some());
        assert_eq!(
            articles[0].thumbnail_url(),
            Some("https://example.com/featured-image.jpg")
        );
        assert!(matches!(
            articles[0].content_source(),
            crate::models::ArticleContentSource::WordpressRest(url)
                if url.as_str() == "https://example.com/wp-json/wp/v2/posts/42?_fields=content.rendered"
        ));
    }

    #[test]
    fn parses_named_html_entities_with_fallback() {
        assert_eq!(
            super::clean_text("<em>Breaking</em>&hellip;"),
            Some("Breaking …".to_string())
        );
    }

    #[test]
    fn falls_back_to_yoast_thumbnail() {
        let articles = super::parse_posts(
            r#"[
                {
                    "link": "https://example.com/news",
                    "title": { "rendered": "Example headline" },
                    "yoast_head_json": {
                        "og_image": [{ "url": "https://example.com/yoast-image.jpg" }]
                    }
                }
            ]"#,
        );

        assert_eq!(
            articles[0].thumbnail_url(),
            Some("https://example.com/yoast-image.jpg")
        );
    }
}
