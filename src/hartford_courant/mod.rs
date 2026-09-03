use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct HartfordCourant;

impl crate::source::Source for HartfordCourant {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(329)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.courant.com")])
    }
}
