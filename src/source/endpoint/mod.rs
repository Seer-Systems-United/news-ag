use crate::parse::Format;
use reqwest::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub enum EndpointScope {
    World,
    US,
    Politics,
    Business,
    Technology,
    Entertainment,
    Sports,
    Science,
    Health,
}

pub struct Endpoint {
    pub url: Url,
    pub format: Format,
    pub scope: EndpointScope,
    pub rules: Vec<crate::parse::rule::Rule>,
}

impl Endpoint {
    #[cfg(not(feature = "async"))]
    pub fn get_articles(&self) -> Vec<crate::models::Article> {
        crate::parse::parse(&self.format, &self.url, &self.rules)
    }

    #[cfg(feature = "async")]
    pub async fn get_articles(&self) -> Vec<crate::models::Article> {
        crate::parse::parse(&self.format, &self.url, &self.rules).await
    }
}
