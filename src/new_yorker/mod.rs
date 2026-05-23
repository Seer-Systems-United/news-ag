use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NewYorker;

impl crate::source::Source for NewYorker {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Entertainment,
            "https://www.newyorker.com/feed/everything",
        )])
    }
}
