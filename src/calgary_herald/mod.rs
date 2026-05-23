use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CalgaryHerald;

impl crate::source::Source for CalgaryHerald {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://calgaryherald.com/feed")])
    }
}
