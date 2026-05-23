use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct WinnipegFreePress;

impl crate::source::Source for WinnipegFreePress {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.winnipegfreepress.com/feed",
        )])
    }
}
