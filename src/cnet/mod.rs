use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Cnet;

impl crate::source::Source for Cnet {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (EndpointScope::Technology, "https://www.cnet.com/rss/news/"),
            (EndpointScope::Health, "https://www.cnet.com/rss/wellness/"),
        ])
    }
}
