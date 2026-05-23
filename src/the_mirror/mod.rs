use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheMirror;

impl crate::source::Source for TheMirror {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.mirror.co.uk/news/?service=rss",
        )])
    }
}
