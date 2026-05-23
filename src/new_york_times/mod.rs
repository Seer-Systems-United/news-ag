use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NewYorkTimes;

impl crate::source::Source for NewYorkTimes {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://rss.nytimes.com/services/xml/rss/nyt/World.xml",
            ),
            (
                EndpointScope::US,
                "https://rss.nytimes.com/services/xml/rss/nyt/US.xml",
            ),
            (
                EndpointScope::Politics,
                "https://rss.nytimes.com/services/xml/rss/nyt/Politics.xml",
            ),
            (
                EndpointScope::Business,
                "https://rss.nytimes.com/services/xml/rss/nyt/Business.xml",
            ),
            (
                EndpointScope::Technology,
                "https://rss.nytimes.com/services/xml/rss/nyt/Technology.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://rss.nytimes.com/services/xml/rss/nyt/Arts.xml",
            ),
            (
                EndpointScope::Sports,
                "https://rss.nytimes.com/services/xml/rss/nyt/Sports.xml",
            ),
            (
                EndpointScope::Science,
                "https://rss.nytimes.com/services/xml/rss/nyt/Science.xml",
            ),
            (
                EndpointScope::Health,
                "https://rss.nytimes.com/services/xml/rss/nyt/Health.xml",
            ),
        ])
    }
}
