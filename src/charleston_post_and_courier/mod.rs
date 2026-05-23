use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CharlestonPostAndCourier;

impl crate::source::Source for CharlestonPostAndCourier {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.postandcourier.com/search/?f=rss",
        )])
    }
}
