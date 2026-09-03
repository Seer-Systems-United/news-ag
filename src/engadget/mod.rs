use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Engadget;

impl crate::source::Source for Engadget {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(203)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(203)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Technology,
            "https://www.engadget.com/rss.xml",
        )])
    }
}
