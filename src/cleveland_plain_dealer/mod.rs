use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct ClevelandPlainDealer;

impl crate::source::Source for ClevelandPlainDealer {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.cleveland.com/arc/outboundfeeds/rss/?outputType=xml",
        )])
    }
}
