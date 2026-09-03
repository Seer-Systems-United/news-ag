use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct CharlestonGazetteMail;

impl crate::source::Source for CharlestonGazetteMail {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(331)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(331)
    }
    fn logo_url() -> &'static str {
        "https://www.wvgazettemail.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::US,
            "https://www.wvgazettemail.com/search/?f=rss",
        )])
    }
}
