use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Atlantic;

impl crate::source::Source for Atlantic {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(801)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(801)
    }
    fn logo_url() -> &'static str {
        "https://www.theatlantic.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.theatlantic.com/feed/all/",
        )])
    }
}
