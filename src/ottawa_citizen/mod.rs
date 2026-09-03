use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct OttawaCitizen;

impl crate::source::Source for OttawaCitizen {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(407)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(407)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://ottawacitizen.com/feed")])
    }
}
