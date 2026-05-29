use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SeattleTimes;

impl crate::source::Source for SeattleTimes {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.seattletimes.com/nation-world/feed/",
            ),
            (
                EndpointScope::US,
                "https://www.seattletimes.com/seattle-news/feed/",
            ),
            (
                EndpointScope::Business,
                "https://www.seattletimes.com/business/feed/",
            ),
            (
                EndpointScope::Technology,
                "https://www.seattletimes.com/business/technology/feed/",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.seattletimes.com/entertainment/feed/",
            ),
            (
                EndpointScope::Sports,
                "https://www.seattletimes.com/sports/feed/",
            ),
        ])
    }
}
