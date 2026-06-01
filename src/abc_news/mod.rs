use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct AbcNews;

impl crate::source::Source for AbcNews {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://feeds.abcnews.com/abcnews/internationalheadlines",
            ),
            (
                EndpointScope::US,
                "https://feeds.abcnews.com/abcnews/usheadlines",
            ),
            (
                EndpointScope::Politics,
                "https://feeds.abcnews.com/abcnews/politicsheadlines",
            ),
            (
                EndpointScope::Business,
                "https://feeds.abcnews.com/abcnews/moneyheadlines",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.abcnews.com/abcnews/technologyheadlines",
            ),
            (
                EndpointScope::Entertainment,
                "https://feeds.abcnews.com/abcnews/entertainmentheadlines",
            ),
            (
                EndpointScope::Sports,
                "https://feeds.abcnews.com/abcnews/sportsheadlines",
            ),
            (
                EndpointScope::Health,
                "https://feeds.abcnews.com/abcnews/healthheadlines",
            ),
        ])
    }
}
