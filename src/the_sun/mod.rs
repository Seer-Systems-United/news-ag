use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheSun;

impl crate::source::Source for TheSun {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://news.google.com/rss/search?q=site:thesun.co.uk&hl=en-US&gl=US&ceid=US:en",
        )])
    }
}
