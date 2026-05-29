use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyBeast;

impl crate::source::Source for DailyBeast {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Politics,
            "http://feeds.feedburner.com/thedailybeast/articles",
        )])
    }
}
