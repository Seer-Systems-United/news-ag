use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct MinneapolisStarTribune;

impl crate::source::Source for MinneapolisStarTribune {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.startribune.com/rss")])
    }
}
