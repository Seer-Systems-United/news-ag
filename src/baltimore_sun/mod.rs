use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct BaltimoreSun;

impl crate::source::Source for BaltimoreSun {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(320)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.baltimoresun.com")])
    }
}
