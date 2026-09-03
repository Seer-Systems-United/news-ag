use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NationalPost;

impl crate::source::Source for NationalPost {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(402)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://nationalpost.com/feed")])
    }
}
