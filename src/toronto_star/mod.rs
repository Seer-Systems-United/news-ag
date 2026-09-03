use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TorontoStar;

impl crate::source::Source for TorontoStar {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(401)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(401)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.thestar.com/search/?f=rss",
        )])
    }
}
