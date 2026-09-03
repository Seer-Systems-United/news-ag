use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NewYorker;

impl crate::source::Source for NewYorker {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(800)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Entertainment,
            "https://www.newyorker.com/feed/everything",
        )])
    }
}
