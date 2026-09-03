use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DemocracyNow;

impl crate::source::Source for DemocracyNow {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(805)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(805)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Politics,
            "https://www.democracynow.org/democracynow.rss",
        )])
    }
}
