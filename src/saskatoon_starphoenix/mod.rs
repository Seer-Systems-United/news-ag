use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SaskatoonStarPhoenix;

impl crate::source::Source for SaskatoonStarPhoenix {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(411)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::World, "https://thestarphoenix.com")])
    }
}
