use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SkyNews;

impl crate::source::Source for SkyNews {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(10)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(10)
    }
    fn logo_url() -> &'static str {
        "https://www.bbc.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://feeds.skynews.com/feeds/rss/world.xml",
            ),
            (
                EndpointScope::US,
                "https://feeds.skynews.com/feeds/rss/us.xml",
            ),
            (
                EndpointScope::Politics,
                "https://feeds.skynews.com/feeds/rss/politics.xml",
            ),
            (
                EndpointScope::Business,
                "https://feeds.skynews.com/feeds/rss/business.xml",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.skynews.com/feeds/rss/technology.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://feeds.skynews.com/feeds/rss/entertainment.xml",
            ),
        ])
    }
}
