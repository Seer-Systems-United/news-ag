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
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/world&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::US,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/national&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::Politics,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/politics&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::Business,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/business&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::Technology,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/technology&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::Entertainment,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/entertainment&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::Sports,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/sports&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::Science,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/health&hl=en-US&gl=US&ceid=US:en",
            ),
            (
                EndpointScope::Health,
                "https://news.google.com/rss/search?q=site:www.washingtonpost.com/health&hl=en-US&gl=US&ceid=US:en",
            ),
        ])
    }
}
