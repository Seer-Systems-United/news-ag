use crate::{
    models::Article,
    parse::json::ssr,
    source::{
        endpoint::{Endpoint, EndpointScope},
        feed,
    },
};

const BASE_URL: &str = "https://www.ajc.com";

pub struct AtlantaJournalConstitution;

impl crate::source::Source for AtlantaJournalConstitution {
    fn endpoints() -> Vec<Endpoint> {
        feed::ssr_json_endpoints(&[(EndpointScope::US, BASE_URL)], parse_articles)
    }
}

fn parse_articles(body: &str) -> Vec<Article> {
    let articles = ssr::next_flight_links(body)
        .into_iter()
        .filter_map(|link| {
            let url = ssr::resolve_url(BASE_URL, &link.href)?;

            if !is_article_url(&url) {
                return None;
            }

            Some(Article::new(link.title, url, None, None, None))
        })
        .collect();

    ssr::dedupe_articles(articles)
}

fn is_article_url(url: &str) -> bool {
    url.starts_with(BASE_URL) && url.contains("/202") && !url.contains("/newsletters/")
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_next_flight_article() {
        let articles = super::parse_articles(
            r#"
            <script>self.__next_f.push([1,"1:[\"$\",\"a\",null,{\"href\":\"/news/2026/05/story/\",\"children\":[\"$\",\"h3\",null,{\"children\":\"Atlanta looks to partner with neighborhood groups\"}]}]"])</script>
            "#,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].url, "https://www.ajc.com/news/2026/05/story/");
        assert_eq!(
            articles[0].title,
            "Atlanta looks to partner with neighborhood groups"
        );
    }
}
