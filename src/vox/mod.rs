use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Vox;

impl crate::source::Source for Vox {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(802)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(802)
    }
    fn logo_url() -> &'static str {
        "https://www.vox.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::World, "https://www.vox.com/rss/index.xml")])
    }
}
