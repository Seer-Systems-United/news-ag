use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Npr;

impl crate::source::Source for Npr {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (EndpointScope::World, "https://feeds.npr.org/1004/rss.xml"),
            (EndpointScope::US, "https://feeds.npr.org/1003/rss.xml"),
            (
                EndpointScope::Politics,
                "https://feeds.npr.org/1014/rss.xml",
            ),
            (
                EndpointScope::Business,
                "https://feeds.npr.org/1006/rss.xml",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.npr.org/1019/rss.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://feeds.npr.org/1007/rss.xml",
            ),
            (EndpointScope::Science, "https://feeds.npr.org/1008/rss.xml"),
            (EndpointScope::Health, "https://feeds.npr.org/1128/rss.xml"),
        ])
    }
}
