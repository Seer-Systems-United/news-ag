use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct FoxNews;

impl crate::source::Source for FoxNews {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://moxie.foxnews.com/google-publisher/world.xml",
            ),
            (
                EndpointScope::US,
                "https://moxie.foxnews.com/google-publisher/us.xml",
            ),
            (
                EndpointScope::Politics,
                "https://moxie.foxnews.com/google-publisher/politics.xml",
            ),
            (
                EndpointScope::Technology,
                "https://moxie.foxnews.com/google-publisher/tech.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://moxie.foxnews.com/google-publisher/entertainment.xml",
            ),
            (
                EndpointScope::Sports,
                "https://moxie.foxnews.com/google-publisher/sports.xml",
            ),
            (
                EndpointScope::Science,
                "https://moxie.foxnews.com/google-publisher/science.xml",
            ),
            (
                EndpointScope::Health,
                "https://moxie.foxnews.com/google-publisher/health.xml",
            ),
        ])
    }
}
