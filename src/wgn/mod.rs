use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Wgn;

impl crate::source::Source for Wgn {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(334)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(334)
    }
    fn logo_url() -> &'static str {
        "https://wgntv.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://wgntv.com/feed/")])
    }
}
