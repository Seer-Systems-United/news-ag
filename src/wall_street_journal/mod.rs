use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct WallStreetJournal;

impl crate::source::Source for WallStreetJournal {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://feeds.a.dj.com/rss/RSSWorldNews.xml",
            ),
            (
                EndpointScope::US,
                "https://feeds.a.dj.com/rss/RSSUSnews.xml",
            ),
            (
                EndpointScope::Business,
                "https://feeds.a.dj.com/rss/WSJcomUSBusiness.xml",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.a.dj.com/rss/RSSWSJD.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://feeds.a.dj.com/rss/RSSLifestyle.xml",
            ),
        ])
    }
}
