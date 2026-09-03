use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NationalPost;

impl crate::source::Source for NationalPost {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(402)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(402)
    }
    fn logo_url() -> &'static str {
        "https://nationalpost.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://nationalpost.com/feed")])
    }
}
