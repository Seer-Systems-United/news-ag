use news_ag::{
    reuters::Reuters,
    source::{Source, endpoint::EndpointScope},
};

#[test]
fn reuters_endpoints_are_defined_and_world_returns_articles() {
    let endpoints = Reuters::endpoints();

    assert!(
        [
            EndpointScope::World,
            EndpointScope::US,
            EndpointScope::Politics,
            EndpointScope::Business,
            EndpointScope::Technology,
            EndpointScope::Entertainment,
            EndpointScope::Sports,
            EndpointScope::Science,
            EndpointScope::Health,
        ]
        .into_iter()
        .all(|scope| endpoints.iter().any(|endpoint| endpoint.scope == scope)),
        "expected reuters to define every supported endpoint scope"
    );

    let articles = Reuters::get_endpoint(EndpointScope::World)
        .expect("reuters should define a world endpoint")
        .get_articles();

    assert!(
        !articles.is_empty(),
        "expected World endpoint to return at least one article"
    );
    assert!(
        articles.iter().any(|article| {
            !article.title.trim().is_empty()
                && article.url.starts_with("https://www.reuters.com/")
                && article.published_at.is_some()
        }),
        "expected at least one World article with title, url, and published_at"
    );
}
