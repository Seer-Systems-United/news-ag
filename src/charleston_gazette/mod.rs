use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CharlestonGazette;

impl crate::source::Source for CharlestonGazette {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(333)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(333)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.wvgazettemail.com/search/?f=rss",
        )])
    }
}
