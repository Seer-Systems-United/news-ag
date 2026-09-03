use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct EdmontonJournal;

impl crate::source::Source for EdmontonJournal {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(405)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(405)
    }
    fn logo_url() -> &'static str {
        "https://edmontonjournal.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://edmontonjournal.com/feed")])
    }
}
