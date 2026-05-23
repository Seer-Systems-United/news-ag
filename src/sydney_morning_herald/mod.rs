use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SydneyMorningHerald;

impl crate::source::Source for SydneyMorningHerald {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://www.smh.com.au/rss/feed.xml")])
    }
}
