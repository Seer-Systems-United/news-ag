use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SydneyMorningHerald;

impl crate::source::Source for SydneyMorningHerald {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(606)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://www.smh.com.au/rss/feed.xml")])
    }
}
