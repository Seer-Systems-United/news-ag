use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Engadget;

impl crate::source::Source for Engadget {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Technology,
            "https://www.engadget.com/rss.xml",
        )])
    }
}
