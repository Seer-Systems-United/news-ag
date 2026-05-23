use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Atlantic;

impl crate::source::Source for Atlantic {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.theatlantic.com/feed/all/",
        )])
    }
}
