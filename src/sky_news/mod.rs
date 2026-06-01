use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SkyNews;

impl crate::source::Source for SkyNews {
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
