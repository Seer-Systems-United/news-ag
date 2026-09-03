use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheAge;

impl crate::source::Source for TheAge {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(607)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(607)
    }
    fn logo_url() -> &'static str {
        "https://www.theage.com.au/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.theage.com.au/rss/feed.xml",
        )])
    }
}
