use news_ag::{
    apnews::ApNews,
    source::{Source, endpoint::EndpointScope},
};

#[test]
fn apnews_world_endpoint_returns_articles() {
    let endpoint =
        ApNews::get_endpoint(EndpointScope::World).expect("apnews should define a world endpoint");

    let articles = endpoint.get_articles();

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
}
