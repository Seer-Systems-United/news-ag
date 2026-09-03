use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Telegraph;

impl crate::source::Source for Telegraph {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(503)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(503)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://www.telegraph.co.uk/rss.xml")])
    }
}
