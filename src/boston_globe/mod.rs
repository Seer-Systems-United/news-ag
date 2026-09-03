use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct BostonGlobe;

impl crate::source::Source for BostonGlobe {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(311)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(311)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.bostonglobe.com/arc/outboundfeeds/rss/?outputType=xml",
        )])
    }
}
