use crate::{
    models::Article,
    parse::json::ssr,
    source::{
        endpoint::{Endpoint, EndpointScope},
        feed,
    },
};
use serde_json::{Map, Value};

const BASE_URL: &str = "https://tyt.com";

pub struct YoungTurks;

impl crate::source::Source for YoungTurks {
    fn endpoints() -> Vec<Endpoint> {
        feed::ssr_json_endpoints(&[(EndpointScope::Politics, BASE_URL)], parse_articles)
    }
}

fn parse_articles(body: &str) -> Vec<Article> {
    let mut articles = Vec::new();

    for value in ssr::json_scripts(body, |id, _| id == "tytapp-state") {
        ssr::visit_objects(&value, &mut |object| {
            if let Some(article) = article_from_object(object) {
                articles.push(article);
            }
        });
    }

    ssr::dedupe_articles(articles)
}

fn article_from_object(object: &Map<String, Value>) -> Option<Article> {
    if ssr::object_string(object, "__backend_type") != Some("cms")
        || ssr::object_string(object, "type") != Some("vod")
    {
        return None;
    }

    let title = ssr::clean_text(ssr::object_string(object, "title")?)?;
    let production_id = ssr::object_string(object, "production_id")?;

    Some(Article::new(
        title,
        format!("{BASE_URL}/watch/{production_id}"),
        ssr::object_string(object, "brand_name").map(|brand| vec![brand.to_string()]),
        ssr::object_string(object, "date").and_then(ssr::parse_date),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_app_state_vod() {
        let articles = super::parse_articles(
            r#"
            <script id="tytapp-state" type="application/json">
                {
                    "topic_items": [
                        {
                            "__backend_type": "cms",
                            "title": "UPDATE: Student confronts school board member",
                            "type": "vod",
                            "production_id": "ab5bd4ec715149fe9",
                            "brand_name": "Indisputable with Dr. Rashad Richey",
                            "date": "2026-05-12T17:00:00.000-07:00"
                        }
                    ]
                }
            </script>
            "#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].url, "https://tyt.com/watch/ab5bd4ec715149fe9");
        assert_eq!(
            articles[0].authors.as_ref().unwrap(),
            &vec!["Indisputable with Dr. Rashad Richey".to_string()]
        );
        assert!(articles[0].published_at.is_some());
    }
}
