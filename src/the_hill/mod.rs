use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheHill;

impl crate::source::Source for TheHill {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(807)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(807)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::Politics, "https://thehill.com/feed/")])
    }
}
