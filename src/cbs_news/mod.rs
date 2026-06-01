use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CbsNews;

impl crate::source::Source for CbsNews {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.cbsnews.com/latest/rss/world",
            ),
            (EndpointScope::US, "https://www.cbsnews.com/latest/rss/us"),
            (
                EndpointScope::Politics,
                "https://www.cbsnews.com/latest/rss/politics",
            ),
            (
                EndpointScope::Business,
                "https://www.cbsnews.com/latest/rss/moneywatch",
            ),
            (
                EndpointScope::Technology,
                "https://www.cbsnews.com/latest/rss/technology",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.cbsnews.com/latest/rss/entertainment",
            ),
            (
                EndpointScope::Science,
                "https://www.cbsnews.com/latest/rss/science",
            ),
            (
                EndpointScope::Health,
                "https://www.cbsnews.com/latest/rss/health",
            ),
        ])
    }
}
