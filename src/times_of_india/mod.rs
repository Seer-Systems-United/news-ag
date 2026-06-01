use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct TimesOfIndia;

impl crate::source::Source for TimesOfIndia {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://timesofindia.indiatimes.com/rssfeeds/296589292.cms",
            ),
            (
                EndpointScope::US,
                "https://timesofindia.indiatimes.com/rssfeeds_us/72258322.cms",
            ),
            (
                EndpointScope::Business,
                "https://timesofindia.indiatimes.com/rssfeeds/1898055.cms",
            ),
            (
                EndpointScope::Technology,
                "https://timesofindia.indiatimes.com/rssfeeds/66949542.cms",
            ),
            (
                EndpointScope::Entertainment,
                "https://timesofindia.indiatimes.com/rssfeeds/1081479906.cms",
            ),
            (
                EndpointScope::Sports,
                "https://timesofindia.indiatimes.com/rssfeeds/4719148.cms",
            ),
            (
                EndpointScope::Science,
                "https://timesofindia.indiatimes.com/rssfeeds/-2128672765.cms",
            ),
        ])
    }
}
