use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheHill;

impl crate::source::Source for TheHill {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::Politics, "https://thehill.com/feed/")])
    }
}
