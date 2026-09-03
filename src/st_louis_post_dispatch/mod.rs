use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct StLouisPostDispatch;

impl crate::source::Source for StLouisPostDispatch {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(319)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(319)
    }
    fn logo_url() -> &'static str {
        "https://www.igorslab.de/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.stltoday.com/search/?f=rss")])
    }
}
