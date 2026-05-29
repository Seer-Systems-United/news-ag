use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CharlestonGazette;

impl crate::source::Source for CharlestonGazette {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.wvgazettemail.com/search/?f=rss",
        )])
    }
}
