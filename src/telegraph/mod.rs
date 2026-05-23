use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Telegraph;

impl crate::source::Source for Telegraph {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://www.telegraph.co.uk/rss.xml")])
    }
}
