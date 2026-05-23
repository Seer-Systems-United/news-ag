use crate::parse::Format;
use reqwest::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn get_articles(&self) -> Vec<crate::models::Article> {
        crate::parse::parse(&self.format, &self.url, &self.rules)
    }
}
