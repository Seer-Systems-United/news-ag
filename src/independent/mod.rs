use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Independent;

impl crate::source::Source for Independent {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(501)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(501)
    }
    fn logo_url() -> &'static str {
        "https://www.independent.co.uk/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.independent.co.uk/news/world/rss",
        )])
    }
}
