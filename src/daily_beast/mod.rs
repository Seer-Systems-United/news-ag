use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyBeast;

impl crate::source::Source for DailyBeast {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(804)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(804)
    }
    fn logo_url() -> &'static str {
        "https://www.thedailybeast.com/favicon.ico"
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::Politics,
            "http://feeds.feedburner.com/thedailybeast/articles",
        )])
    }
}
