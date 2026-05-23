use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct HoustonChronicle;

impl crate::source::Source for HoustonChronicle {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.houstonchronicle.com/rss/feed/News-270.php",
        )])
    }
}
