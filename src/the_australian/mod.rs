use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TheAustralian;

impl crate::source::Source for TheAustralian {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(608)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(608)
    }
    fn logo_url() -> &'static str {
        "https://www.theaustralian.com.au/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://news.google.com/rss/search?q=site:www.theaustralian.com.au&hl=en-US&gl=US&ceid=US:en",
        )])
    }
}
