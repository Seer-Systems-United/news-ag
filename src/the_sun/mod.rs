use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheSun;

impl crate::source::Source for TheSun {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(504)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(504)
    }
    fn logo_url() -> &'static str {
        "https://www.thesun.co.uk/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://news.google.com/rss/search?q=site:thesun.co.uk&hl=en-US&gl=US&ceid=US:en",
        )])
    }
}
