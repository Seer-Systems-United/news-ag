use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct GlobeAndMail;

impl crate::source::Source for GlobeAndMail {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.theglobeandmail.com/arc/outboundfeeds/rss/category/world/?outputType=xml",
        )])
    }
}
