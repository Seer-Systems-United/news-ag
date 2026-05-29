use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CharlestonGazetteMail;

impl crate::source::Source for CharlestonGazetteMail {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.wvgazettemail.com/search/?f=rss",
        )])
    }
}
