use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyMail;

impl crate::source::Source for DailyMail {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(506)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(506)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.dailymail.co.uk/news/index.rss",
        )])
    }
}
