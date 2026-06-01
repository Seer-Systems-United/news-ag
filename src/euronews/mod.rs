use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Euronews;

impl crate::source::Source for Euronews {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.euronews.com/rss?level=theme&name=news",
            ),
            (
                EndpointScope::Business,
                "https://www.euronews.com/rss?level=theme&name=business",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.euronews.com/rss?level=theme&name=culture",
            ),
        ])
    }
}
