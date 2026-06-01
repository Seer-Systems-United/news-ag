use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct StraitsTimes;

impl crate::source::Source for StraitsTimes {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.straitstimes.com/news/world/rss.xml",
            ),
            (
                EndpointScope::Business,
                "https://www.straitstimes.com/news/business/rss.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.straitstimes.com/news/life/rss.xml",
            ),
            (
                EndpointScope::Sports,
                "https://www.straitstimes.com/news/sport/rss.xml",
            ),
        ])
    }
}
