use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Guardian;

impl crate::source::Source for Guardian {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.theguardian.com/world/rss",
            ),
            (EndpointScope::US, "https://www.theguardian.com/us-news/rss"),
            (
                EndpointScope::Politics,
                "https://www.theguardian.com/us-news/us-politics/rss",
            ),
            (
                EndpointScope::Business,
                "https://www.theguardian.com/business/rss",
            ),
            (
                EndpointScope::Technology,
                "https://www.theguardian.com/technology/rss",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.theguardian.com/culture/rss",
            ),
            (
                EndpointScope::Sports,
                "https://www.theguardian.com/sport/rss",
            ),
            (
                EndpointScope::Science,
                "https://www.theguardian.com/science/rss",
            ),
            (
                EndpointScope::Health,
                "https://www.theguardian.com/society/health/rss",
            ),
        ])
    }
}
