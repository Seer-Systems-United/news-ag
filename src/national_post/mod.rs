use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NationalPost;

impl crate::source::Source for NationalPost {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://nationalpost.com/feed")])
    }
}
