use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyCaller;

impl crate::source::Source for DailyCaller {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(808)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(808)
    }
    fn logo_url() -> &'static str {
        "https://www.thedailycaller.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::Politics, "https://dailycaller.com/feed/")])
    }
}
