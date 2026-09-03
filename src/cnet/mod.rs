use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Cnet;

impl crate::source::Source for Cnet {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(211)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(211)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (EndpointScope::Technology, "https://www.cnet.com/rss/news/"),
            (EndpointScope::Health, "https://www.cnet.com/rss/wellness/"),
        ])
    }
}
