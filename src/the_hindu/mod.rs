use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheHindu;

impl crate::source::Source for TheHindu {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(615)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(615)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.thehindu.com/news/international/feeder/default.rss",
            ),
            (
                EndpointScope::Business,
                "https://www.thehindu.com/business/feeder/default.rss",
            ),
            (
                EndpointScope::Technology,
                "https://www.thehindu.com/sci-tech/technology/feeder/default.rss",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.thehindu.com/entertainment/feeder/default.rss",
            ),
            (
                EndpointScope::Sports,
                "https://www.thehindu.com/sport/feeder/default.rss",
            ),
            (
                EndpointScope::Science,
                "https://www.thehindu.com/sci-tech/science/feeder/default.rss",
            ),
            (
                EndpointScope::Health,
                "https://www.thehindu.com/sci-tech/health/feeder/default.rss",
            ),
        ])
    }
}
