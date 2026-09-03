use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheVerge;

impl crate::source::Source for TheVerge {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(204)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(204)
    }
    fn logo_url() -> &'static str {
        "https://www.businessinsider.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Technology,
            "https://www.theverge.com/rss/index.xml",
        )])
    }
}
