use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct WinnipegFreePress;

impl crate::source::Source for WinnipegFreePress {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(408)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.winnipegfreepress.com/feed",
        )])
    }
}
