use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct WashingtonPost;

impl crate::source::Source for WashingtonPost {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://feeds.washingtonpost.com/rss/world",
            ),
            (
                EndpointScope::US,
                "https://feeds.washingtonpost.com/rss/national",
            ),
            (
                EndpointScope::Politics,
                "https://feeds.washingtonpost.com/rss/politics",
            ),
            (
                EndpointScope::Business,
                "https://feeds.washingtonpost.com/rss/business",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.washingtonpost.com/rss/business/technology",
            ),
            (
                EndpointScope::Entertainment,
                "https://feeds.washingtonpost.com/rss/entertainment",
            ),
            (
                EndpointScope::Sports,
                "https://feeds.washingtonpost.com/rss/sports",
            ),
            (
                EndpointScope::Science,
                "https://feeds.washingtonpost.com/rss/national/health-science",
            ),
            (
                EndpointScope::Health,
                "https://feeds.washingtonpost.com/rss/national/health-science",
            ),
        ])
    }
}
