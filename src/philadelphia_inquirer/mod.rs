use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct PhiladelphiaInquirer;

impl crate::source::Source for PhiladelphiaInquirer {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.inquirer.com/arc/outboundfeeds/rss/?outputType=xml",
        )])
    }
}
