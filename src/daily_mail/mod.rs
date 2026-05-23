use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct DailyMail;

impl crate::source::Source for DailyMail {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[(
            EndpointScope::World,
            "https://www.dailymail.co.uk/news/index.rss",
        )])
    }
}
