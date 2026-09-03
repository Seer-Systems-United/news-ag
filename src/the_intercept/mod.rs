use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheIntercept;

impl crate::source::Source for TheIntercept {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(803)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::Politics, "https://theintercept.com/feed/")])
    }
}
