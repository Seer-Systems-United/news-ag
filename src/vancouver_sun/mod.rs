use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct VancouverSun;

impl crate::source::Source for VancouverSun {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(403)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://vancouversun.com/feed")])
    }
}
