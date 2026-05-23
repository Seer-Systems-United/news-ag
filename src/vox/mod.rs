use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Vox;

impl crate::source::Source for Vox {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://www.vox.com/rss/index.xml")])
    }
}
