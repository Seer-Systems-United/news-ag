use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheBlaze;

impl crate::source::Source for TheBlaze {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Politics,
            "https://www.theblaze.com/feeds/feed.rss",
        )])
    }
}
