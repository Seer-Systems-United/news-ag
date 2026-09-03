use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct HartfordCourant;

impl crate::source::Source for HartfordCourant {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(329)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(329)
    }
    fn logo_url() -> &'static str {
        "https://www.courant.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.courant.com")])
    }
}
