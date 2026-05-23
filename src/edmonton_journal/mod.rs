use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct EdmontonJournal;

impl crate::source::Source for EdmontonJournal {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://edmontonjournal.com/feed")])
    }
}
