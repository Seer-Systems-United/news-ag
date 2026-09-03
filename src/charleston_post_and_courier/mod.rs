use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CharlestonPostAndCourier;

impl crate::source::Source for CharlestonPostAndCourier {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(332)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(332)
    }
    fn logo_url() -> &'static str {
        "https://www.postandcourier.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.postandcourier.com/search/?f=rss",
        )])
    }
}
