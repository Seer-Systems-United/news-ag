use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyTelegraph;

impl crate::source::Source for DailyTelegraph {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.dailytelegraph.com.au/news/rss",
        )])
    }
}
