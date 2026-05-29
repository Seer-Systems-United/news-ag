use crate::{
    models::Article,
    parse::json::ssr,
    source::{
        endpoint::{Endpoint, EndpointScope},
        feed,
    },
};
use serde_json::{Map, Value};

const BASE_URL: &str = "https://www.dallasnews.com";

pub struct DallasMorningNews;

impl crate::source::Source for DallasMorningNews {
    fn endpoints() -> Vec<Endpoint> {
        feed::ssr_json_endpoints(&[(EndpointScope::US, BASE_URL)], parse_articles)
    }
}

fn parse_articles(body: &str) -> Vec<Article> {
    let mut articles = Vec::new();

    for value in ssr::json_scripts(body, |id, script_type| {
        id == "__NEXT_DATA__" || ssr::is_json_script_type(script_type)
    }) {
        ssr::visit_objects(&value, &mut |object| {
            if let Some(article) = article_from_object(object) {
                articles.push(article);
            }
        });
    }

    ssr::dedupe_articles(articles)
}

fn article_from_object(object: &Map<String, Value>) -> Option<Article> {
    ssr::article_from_object(object, BASE_URL, &["title", "headline"], |object, url| {
        ssr::object_string(object, "type") == Some("article")
            || ssr::object_string(object, "__typename") == Some("ArticleStub")
            || url.contains("/article/")
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_next_data_article() {
        let articles = super::parse_articles(
            r#"
            <script id="__NEXT_DATA__" type="application/json">
                {
                    "props": {
                        "items": [
                            {
                                "__typename": "ArticleStub",
                                "type": "article",
                                "title": "Small break in gas prices arrives for Dallas drivers",
                                "url": "/news/transportation/article/test.php",
                                "displayedDate": "2026-05-28 13:42:34",
                                "authors": [{ "name": "Jane Reporter" }]
                            }
                        ]
                    }
                }
            </script>
            "#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(
            articles[0].title,
            "Small break in gas prices arrives for Dallas drivers"
        );
        assert_eq!(
            articles[0].url,
            "https://www.dallasnews.com/news/transportation/article/test.php"
        );
        assert_eq!(
            articles[0].authors.as_ref().unwrap(),
            &vec!["Jane Reporter".to_string()]
        );
        assert!(articles[0].published_at.is_some());
    }
}
