use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct PittsburghPostGazette;

impl crate::source::Source for PittsburghPostGazette {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(317)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(317)
    }
    fn logo_url() -> &'static str {
        "https://www.techradar.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.post-gazette.com/rss")])
    }
}
