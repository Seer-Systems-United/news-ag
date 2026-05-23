use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct BostonGlobe;

impl crate::source::Source for BostonGlobe {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.bostonglobe.com/arc/outboundfeeds/rss/?outputType=xml",
        )])
    }
}
