use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct RichmondTimesDispatch;

impl crate::source::Source for RichmondTimesDispatch {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(328)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(328)
    }
    fn logo_url() -> &'static str {
        "https://richmond.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://richmond.com/search/?f=rss")])
    }
}
