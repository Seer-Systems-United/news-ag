use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheAge;

impl crate::source::Source for TheAge {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.theage.com.au/rss/feed.xml",
        )])
    }
}
