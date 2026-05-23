use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct Cnn;

impl crate::source::Source for Cnn {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "http://rss.cnn.com/rss/edition_world.rss",
            ),
            (EndpointScope::US, "http://rss.cnn.com/rss/edition_us.rss"),
            (
                EndpointScope::Politics,
                "http://rss.cnn.com/rss/cnn_allpolitics.rss",
            ),
            (
                EndpointScope::Business,
                "http://rss.cnn.com/rss/money_news_international.rss",
            ),
            (
                EndpointScope::Technology,
                "http://rss.cnn.com/rss/edition_technology.rss",
            ),
            (
                EndpointScope::Entertainment,
                "http://rss.cnn.com/rss/edition_entertainment.rss",
            ),
            (
                EndpointScope::Sports,
                "http://rss.cnn.com/rss/edition_sport.rss",
            ),
            (
                EndpointScope::Science,
                "http://rss.cnn.com/rss/edition_space.rss",
            ),
            (
                EndpointScope::Health,
                "http://rss.cnn.com/rss/edition_health.rss",
            ),
        ])
    }
}
