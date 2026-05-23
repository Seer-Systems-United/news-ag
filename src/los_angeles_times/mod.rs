use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct LosAngelesTimes;

impl crate::source::Source for LosAngelesTimes {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.latimes.com/rss2.0.xml")])
    }
}
