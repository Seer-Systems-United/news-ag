use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct ReginaLeaderPost;

impl crate::source::Source for ReginaLeaderPost {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://leaderpost.com/feed")])
    }
}
