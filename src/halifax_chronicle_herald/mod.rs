use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct HalifaxChronicleHerald;

impl crate::source::Source for HalifaxChronicleHerald {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(409)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.saltwire.com/category/nova-scotia/feed.xml",
        )])
    }
}
