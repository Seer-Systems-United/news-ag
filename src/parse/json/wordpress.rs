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

pub(crate) fn parse_posts(body: &str) -> Vec<Article> {
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
    let fragment = scraper::Html::parse_fragment(value);
    let text = fragment.root_element().text().collect::<Vec<_>>().join(" ");
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");

    if text.is_empty() { None } else { Some(text) }
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
}
