use news_ag::{
    DenverPost,
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

#[cfg(not(feature = "async"))]
fn get_content(article: &Article) -> String {
    article.get_content().unwrap()
}

#[cfg(feature = "async")]
fn get_content(article: &Article) -> String {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { article.get_content().await.unwrap() })
}

#[test]
fn wordpress_article_returns_plain_text_content() {
    let endpoint =
        DenverPost::get_endpoint(EndpointScope::US).expect("denver post should define an endpoint");
    let articles = get_articles(&endpoint);
    let article = articles[3].clone();
    let content = get_content(&article);

    println!("Content returned from Denver Post article: {}", &content);

    assert!(
        content.split_whitespace().count() > 10,
        "expected WordPress article content to contain meaningful text"
    );
    assert!(
        !content.contains("<p>"),
        "expected WordPress article content to be plain text"
    );
}
