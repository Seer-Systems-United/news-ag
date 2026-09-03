use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TechCrunch;

impl crate::source::Source for TechCrunch {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(202)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::Technology, "https://techcrunch.com/feed/")])
    }
}
