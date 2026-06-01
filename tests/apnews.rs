use news_ag::{
    ApNews,
    models::Article,
    source::{
        Source,
        endpoint::{Endpoint, EndpointScope},
    },
};

#[cfg(not(feature = "async"))]
fn get_articles(endpoint: &Endpoint) -> Vec<Article> {
    endpoint.get_articles()
}

#[cfg(feature = "async")]
fn get_articles(endpoint: &Endpoint) -> Vec<Article> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { endpoint.get_articles().await })
}

#[test]
fn apnews_world_endpoint_returns_articles() {
    let endpoint =
        ApNews::get_endpoint(EndpointScope::World).expect("apnews should define a world endpoint");

    let articles = get_articles(&endpoint);

    dbg!("Articles returned from APNews endpoint: {:#?}", &articles);

    assert!(
        !articles.is_empty(),
        "expected apnews endpoint to return at least one article"
    );
    assert!(
        articles
            .iter()
            .any(|article| !article.title.trim().is_empty()),
        "expected at least one apnews article with a title"
    );
    assert!(
        articles
            .iter()
            .any(|article| article.thumbnail_url().is_some()),
        "expected at least one apnews article with a thumbnail"
    );
}
