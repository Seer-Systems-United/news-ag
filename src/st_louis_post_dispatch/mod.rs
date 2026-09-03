use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct StLouisPostDispatch;

impl crate::source::Source for StLouisPostDispatch {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(319)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://www.stltoday.com/search/?f=rss")])
    }
}
