use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheAustralian;

impl crate::source::Source for TheAustralian {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.theaustralian.com.au/news/rss",
        )])
    }
}
