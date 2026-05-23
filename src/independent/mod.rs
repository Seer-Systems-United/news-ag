use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Independent;

impl crate::source::Source for Independent {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.independent.co.uk/news/world/rss",
        )])
    }
}
