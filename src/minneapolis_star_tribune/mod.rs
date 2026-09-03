use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct MinneapolisStarTribune;

impl crate::source::Source for MinneapolisStarTribune {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(313)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(313)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.startribune.com/rss")])
    }
}
