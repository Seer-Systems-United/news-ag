use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TorontoStar;

impl crate::source::Source for TorontoStar {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.thestar.com/search/?f=rss",
        )])
    }
}
