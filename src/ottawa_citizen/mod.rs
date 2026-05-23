use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct OttawaCitizen;

impl crate::source::Source for OttawaCitizen {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://ottawacitizen.com/feed")])
    }
}
