use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct StJohnsTelegram;

impl crate::source::Source for StJohnsTelegram {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.saltwire.com/category/newfoundland-labrador/feed",
        )])
    }
}
