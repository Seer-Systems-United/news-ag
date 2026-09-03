use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TampaBayTimes;

impl crate::source::Source for TampaBayTimes {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(315)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.tampabay.com/arc/outboundfeeds/rss/?outputType=xml",
        )])
    }
}
