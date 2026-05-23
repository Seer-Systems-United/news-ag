use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Bloomberg;

impl crate::source::Source for Bloomberg {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::Business,
                "https://feeds.bloomberg.com/markets/news.rss",
            ),
            (
                EndpointScope::Politics,
                "https://feeds.bloomberg.com/politics/news.rss",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.bloomberg.com/technology/news.rss",
            ),
        ])
    }
}
