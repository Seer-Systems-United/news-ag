use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Wgn;

impl crate::source::Source for Wgn {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://wgntv.com/feed/")])
    }
}
