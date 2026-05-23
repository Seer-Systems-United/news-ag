use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheIntercept;

impl crate::source::Source for TheIntercept {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::Politics, "https://theintercept.com/feed/")])
    }
}
