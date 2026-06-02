//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
pub use news_ag::models::Article;
#[cfg(feature = "server")]
use news_ag::{
    source::{endpoint::EndpointScope, Source},
    AlJazeera, ApNews, Npr, Reuters,
};

/// Get articles from the AP news API.
#[get("/api/ap-news/get-articles")]
pub async fn ap_get_articles() -> Result<Vec<Article>, ServerFnError> {
    let endpoint = ApNews::get_endpoint(EndpointScope::World)
        .ok_or_else(|| ServerFnError::new("AP News does not define a world endpoint"))?;

    Ok(endpoint.get_articles().await)
}

/// Get articles from the Reuters news API.
#[get("/api/reuters/get-articles")]
pub async fn reuters_get_articles() -> Result<Vec<Article>, ServerFnError> {
    let endpoint = Reuters::get_endpoint(EndpointScope::World)
        .ok_or_else(|| ServerFnError::new("Reuters does not define a world endpoint"))?;

    Ok(endpoint.get_articles().await)
}

/// Get articles from the NPR news API.
#[get("/api/npr/get-articles")]
pub async fn npr_get_articles() -> Result<Vec<Article>, ServerFnError> {
    let endpoint = Npr::get_endpoint(EndpointScope::World)
        .ok_or_else(|| ServerFnError::new("NPR does not define a world endpoint"))?;

    Ok(endpoint.get_articles().await)
}

// Get articles from the Al Jazeera news API.
#[get("/api/al-jazeera/get-articles")]
pub async fn al_jazeera_get_articles() -> Result<Vec<Article>, ServerFnError> {
    let endpoint = AlJazeera::get_endpoint(EndpointScope::World)
        .ok_or_else(|| ServerFnError::new("Al Jazeera does not define a world endpoint"))?;

    Ok(endpoint.get_articles().await)
}
