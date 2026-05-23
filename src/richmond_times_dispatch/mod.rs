use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct RichmondTimesDispatch;

impl crate::source::Source for RichmondTimesDispatch {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(EndpointScope::US, "https://richmond.com/search/?f=rss")])
    }
}
