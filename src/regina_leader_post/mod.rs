use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct ReginaLeaderPost;

impl crate::source::Source for ReginaLeaderPost {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(410)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://leaderpost.com/feed")])
    }
}
