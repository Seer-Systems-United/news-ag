use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheSun;

impl crate::source::Source for TheSun {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://www.thesun.co.uk/news/feed/")])
    }
}
