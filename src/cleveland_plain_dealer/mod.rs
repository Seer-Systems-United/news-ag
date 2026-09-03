use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct ClevelandPlainDealer;

impl crate::source::Source for ClevelandPlainDealer {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(314)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(314)
    }
    fn logo_url() -> &'static str {
        "https://www.extremetech.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.cleveland.com/arc/outboundfeeds/rss/?outputType=xml",
        )])
    }
}
