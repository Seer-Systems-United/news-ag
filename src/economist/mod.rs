use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Economist;

impl crate::source::Source for Economist {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.economist.com/latest/rss.xml",
            ),
            (
                EndpointScope::US,
                "https://www.economist.com/united-states/rss.xml",
            ),
            (
                EndpointScope::Politics,
                "https://www.economist.com/united-states/rss.xml",
            ),
            (
                EndpointScope::Business,
                "https://www.economist.com/business/rss.xml",
            ),
            (
                EndpointScope::Technology,
                "https://www.economist.com/science-and-technology/rss.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.economist.com/culture/rss.xml",
            ),
            (
                EndpointScope::Science,
                "https://www.economist.com/science-and-technology/rss.xml",
            ),
        ])
    }
}
