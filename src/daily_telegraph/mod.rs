use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyTelegraph;

impl crate::source::Source for DailyTelegraph {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(605)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(605)
    }
    fn logo_url() -> &'static str {
        "https://www.dailytelegraph.com.au/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://news.google.com/rss/search?q=site:dailytelegraph.com.au&hl=en-US&gl=US&ceid=US:en",
        )])
    }
}
