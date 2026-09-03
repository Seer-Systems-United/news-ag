use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DenverPost;

impl crate::source::Source for DenverPost {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(309)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.denverpost.com")])
    }
}
