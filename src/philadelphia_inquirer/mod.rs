use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct PhiladelphiaInquirer;

impl crate::source::Source for PhiladelphiaInquirer {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(306)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(306)
    }
    fn logo_url() -> &'static str {
        "https://www.gizmodo.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.inquirer.com/arc/outboundfeeds/rss/?outputType=xml",
        )])
    }
}
