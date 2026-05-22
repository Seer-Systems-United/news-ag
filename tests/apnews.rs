use news_sources::{apnews::ApNews, source::Source};

#[test]
fn apnews_world_endpoint_returns_articles() {
    let endpoint = ApNews
        .endpoints()
        .into_iter()
        .next()
        .expect("apnews should define at least one endpoint");

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
