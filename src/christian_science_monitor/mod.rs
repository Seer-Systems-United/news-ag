use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct ChristianScienceMonitor;

impl crate::source::Source for ChristianScienceMonitor {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://rss.csmonitor.com/feeds/world",
            ),
            (EndpointScope::US, "https://rss.csmonitor.com/feeds/usa"),
            (
                EndpointScope::Politics,
                "https://rss.csmonitor.com/feeds/politics",
            ),
            (
                EndpointScope::Science,
                "https://rss.csmonitor.com/feeds/science",
            ),
        ])
    }
}
