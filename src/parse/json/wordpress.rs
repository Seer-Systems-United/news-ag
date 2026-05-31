use std::borrow::Cow;

use crate::models::Article;

#[derive(Debug, serde::Deserialize)]
struct WordpressPost {
    date: Option<String>,
    date_gmt: Option<String>,
    link: Option<String>,
    title: Option<RenderedText>,
    yoast_head_json: Option<YoastHeadJson>,
}

#[derive(Debug, serde::Deserialize)]
struct RenderedText {
    rendered: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct YoastHeadJson {
    author: Option<String>,
}

pub fn parse_posts(body: &str) -> Vec<Article> {
    serde_json::from_str::<Vec<WordpressPost>>(body)
        .unwrap_or_default()
        .into_iter()
        .filter_map(article_from_post)
        .collect()
}

fn article_from_post(post: WordpressPost) -> Option<Article> {
    Some(Article {
        title: clean_text(&post.title?.rendered?)?,
        url: text(post.link.as_deref())?,
        authors: authors(post.yoast_head_json),
        published_at: post
            .date_gmt
            .as_deref()
            .and_then(parse_wordpress_date)
            .or_else(|| post.date.as_deref().and_then(parse_wordpress_date)),
    })
}

fn authors(yoast_head_json: Option<YoastHeadJson>) -> Option<Vec<String>> {
    let author = clean_text(yoast_head_json?.author.as_deref()?)?;
    Some(vec![author])
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
                    "yoast_head_json": { "author": "Jane Reporter" }
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
    }

    #[test]
    fn parses_named_html_entities_with_fallback() {
        assert_eq!(
            super::clean_text("<em>Breaking</em>&hellip;"),
            Some("Breaking …".to_string())
        );
    }
}
