use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct BaltimoreSun;

impl crate::source::Source for BaltimoreSun {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(320)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(320)
    }
    fn logo_url() -> &'static str {
        "https://www.techspot.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::wordpress_endpoints(&[(EndpointScope::US, "https://www.baltimoresun.com")])
    }
}
