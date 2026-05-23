use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheVerge;

impl crate::source::Source for TheVerge {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Technology,
            "https://www.theverge.com/rss/index.xml",
        )])
    }
}
