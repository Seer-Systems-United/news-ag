use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DemocracyNow;

impl crate::source::Source for DemocracyNow {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Politics,
            "https://www.democracynow.org/democracynow.rss",
        )])
    }
}
