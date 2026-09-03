use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct AlJazeera;

impl crate::source::Source for AlJazeera {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(13)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.aljazeera.com/xml/rss/all.xml",
        )])
    }
}
