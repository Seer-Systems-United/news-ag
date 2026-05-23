use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SaskatoonStarPhoenix;

impl crate::source::Source for SaskatoonStarPhoenix {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://thestarphoenix.com/feed")])
    }
}
