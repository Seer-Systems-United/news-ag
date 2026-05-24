use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheAustralian;

impl crate::source::Source for TheAustralian {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://news.google.com/rss/search?q=site:theaustralian.com.au&hl=en-US&gl=US&ceid=US:en",
        )])
    }
}
