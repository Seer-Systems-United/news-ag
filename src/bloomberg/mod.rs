use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Bloomberg;

impl crate::source::Source for Bloomberg {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(102)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(102)
    }
    fn logo_url() -> &'static str {
        "https://www.wamu.org/favicon.ico"
    }
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
