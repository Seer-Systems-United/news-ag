use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyTelegraph;

impl crate::source::Source for DailyTelegraph {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://news.google.com/rss/search?q=site:dailytelegraph.com.au&hl=en-US&gl=US&ceid=US:en",
        )])
    }
}
