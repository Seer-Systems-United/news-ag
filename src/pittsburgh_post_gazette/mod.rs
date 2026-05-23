use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct PittsburghPostGazette;

impl crate::source::Source for PittsburghPostGazette {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.post-gazette.com/rss")])
    }
}
