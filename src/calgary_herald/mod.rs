use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CalgaryHerald;

impl crate::source::Source for CalgaryHerald {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(404)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://calgaryherald.com/feed")])
    }
}
