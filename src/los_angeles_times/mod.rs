use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct LosAngelesTimes;

impl crate::source::Source for LosAngelesTimes {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(303)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(303)
    }
    fn logo_url() -> &'static str {
        "https://arstechnica.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.latimes.com/rss2.0.xml")])
    }
}
