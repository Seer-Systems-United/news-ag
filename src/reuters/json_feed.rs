use crate::models::Article;

pub fn parse(body: &str) -> Option<Vec<Article>> {
    let response: serde_json::Value = serde_json::from_str(body).ok()?;

    response
        .pointer("/result/articles")
        .and_then(serde_json::Value::as_array)
        .map(|articles| articles.iter().filter_map(parse_article).collect())
}

fn parse_article(article: &serde_json::Value) -> Option<Article> {
    Some(Article::new(
        article_title(article)?,
        super::helpers::absolute_url(article_path(article)?),
        article_authors(article),
        article_published_at(article),
    ))
}

fn article_title(article: &serde_json::Value) -> Option<String> {
    article
        .get("title")
        .or_else(|| article.get("basic_headline"))
        .or_else(|| article.get("web"))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .map(str::to_string)
}

fn article_path(article: &serde_json::Value) -> Option<&str> {
    article
        .get("canonical_url")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|url| !url.is_empty())
}

fn article_authors(article: &serde_json::Value) -> Option<Vec<String>> {
    article
        .get("authors")
        .and_then(serde_json::Value::as_array)
        .and_then(|authors| {
            let names = authors
                .iter()
                .filter_map(author_name)
                .map(str::to_string)
                .collect::<Vec<_>>();

            if names.is_empty() { None } else { Some(names) }
        })
}

fn author_name(author: &serde_json::Value) -> Option<&str> {
    author
        .get("byline")
        .or_else(|| author.get("name"))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
}

fn article_published_at(article: &serde_json::Value) -> Option<chrono::DateTime<chrono::Utc>> {
    article
        .get("published_time")
        .or_else(|| article.get("display_time"))
        .and_then(serde_json::Value::as_str)
        .and_then(super::helpers::parse_date)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_articles_from_reuters_json() {
        let articles = super::parse(
            r#"{
                "statusCode": 200,
                "message": "Success",
                "result": {
                    "articles": [
                        {
                            "canonical_url": "/world/us/one-dead-36-injured-explosion-new-york-dry-dock-2026-05-23/",
                            "title": "One dead, 36 injured in explosion at New York dry dock",
                            "published_time": "2026-05-23T01:02:48.849Z",
                            "authors": [
                                { "byline": "Reuters" }
                            ]
                        }
                    ]
                }
            }"#,
        )
        .expect("json feed should parse");

        assert_eq!(articles.len(), 1);
        assert_eq!(
            articles[0].title,
            "One dead, 36 injured in explosion at New York dry dock"
        );
        assert_eq!(
            articles[0].url,
            "https://www.reuters.com/world/us/one-dead-36-injured-explosion-new-york-dry-dock-2026-05-23/"
        );
        assert_eq!(
            articles[0].authors.as_ref().unwrap(),
            &vec!["Reuters".to_string()]
        );
        assert!(articles[0].published_at.is_some());
    }
}
