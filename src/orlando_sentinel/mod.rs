use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct OrlandoSentinel;

impl crate::source::Source for OrlandoSentinel {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(316)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.orlandosentinel.com")])
    }
}
