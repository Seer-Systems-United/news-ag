use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheMirror;

impl crate::source::Source for TheMirror {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(505)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(505)
    }
    fn logo_url() -> &'static str {
        "https://www.mirror.co.uk/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.mirror.co.uk/news/?service=rss",
        )])
    }
}
