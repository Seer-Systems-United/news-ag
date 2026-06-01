use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct JapanTimes;

impl crate::source::Source for JapanTimes {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.japantimes.co.jp/world/feed/",
            ),
            (
                EndpointScope::Business,
                "https://www.japantimes.co.jp/business/feed/",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.japantimes.co.jp/culture/feed/",
            ),
            (
                EndpointScope::Sports,
                "https://www.japantimes.co.jp/sports/feed/",
            ),
        ])
    }
}
