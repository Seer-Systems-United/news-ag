//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
pub use news_ag::models::Article;
#[cfg(feature = "server")]
use news_ag::{
    source::{endpoint::EndpointScope, Source},
    ApNews,
};

/// Get articles from the AP news API.
#[get("/api/ap-news/get-articles")]
pub async fn ap_news_get_articles() -> Result<Vec<Article>, ServerFnError> {
    let endpoint = ApNews::get_endpoint(EndpointScope::World)
        .ok_or_else(|| ServerFnError::new("AP News does not define a world endpoint"))?;

    Ok(endpoint.get_articles().await)
}
