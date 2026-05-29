use crate::{
    models::Article,
    parse::json::ssr,
    source::{
        endpoint::{Endpoint, EndpointScope},
        feed,
    },
};
use serde_json::{Map, Value};

const BASE_URL: &str = "https://www.thetimes.com";

pub struct TheTimes;

impl crate::source::Source for TheTimes {
    fn endpoints() -> Vec<Endpoint> {
        feed::ssr_json_endpoints(&[(EndpointScope::World, BASE_URL)], parse_articles)
    }
}

fn parse_articles(body: &str) -> Vec<Article> {
    let mut articles = Vec::new();
    let mut values =
        ssr::json_scripts(body, |_, script_type| ssr::is_json_script_type(script_type));

    values.extend(ssr::json_assignments(body, "window.__TIMES_STATE__ = "));

    for value in values {
        ssr::visit_objects(&value, &mut |object| {
            if let Some(article) = article_from_object(object) {
                articles.push(article);
            }
        });
    }

    ssr::dedupe_articles(articles)
}

fn article_from_object(object: &Map<String, Value>) -> Option<Article> {
    ssr::article_from_object(object, BASE_URL, &["headline", "title"], |_, url| {
        is_article_url(url)
    })
}

fn is_article_url(url: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(url) else {
        return false;
    };

    if url.domain() != Some("www.thetimes.com") {
        return false;
    }

    let path = url.path();
    path.contains("/article/")
        || path.starts_with("/video/")
        || path.starts_with("/travel/destinations/")
        || path.starts_with("/travel/advice/")
        || path.starts_with("/recipes/")
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_times_state_article() {
        let articles = super::parse_articles(
            r#"
            <script>
                window.__TIMES_STATE__ = {
                    "preloadedData": {
                        "headline": "Andy Burnham calls for more state control",
                        "url": "/uk/politics/article/andy-burnham-response-tony-blair-6thjcb8ss"
                    }
                };
            </script>
            "#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(
            articles[0].url,
            "https://www.thetimes.com/uk/politics/article/andy-burnham-response-tony-blair-6thjcb8ss"
        );
    }
}
