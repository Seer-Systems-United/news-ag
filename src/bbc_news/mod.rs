use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct BbcNews;

impl crate::source::Source for BbcNews {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://feeds.bbci.co.uk/news/world/rss.xml",
            ),
            (
                EndpointScope::US,
                "https://feeds.bbci.co.uk/news/world/us_and_canada/rss.xml",
            ),
            (
                EndpointScope::Politics,
                "https://feeds.bbci.co.uk/news/politics/rss.xml",
            ),
            (
                EndpointScope::Business,
                "https://feeds.bbci.co.uk/news/business/rss.xml",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.bbci.co.uk/news/technology/rss.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://feeds.bbci.co.uk/news/entertainment_and_arts/rss.xml",
            ),
            (
                EndpointScope::Sports,
                "https://feeds.bbci.co.uk/sport/rss.xml",
            ),
            (
                EndpointScope::Science,
                "https://feeds.bbci.co.uk/news/science_and_environment/rss.xml",
            ),
            (
                EndpointScope::Health,
                "https://feeds.bbci.co.uk/news/health/rss.xml",
            ),
        ])
    }
}
