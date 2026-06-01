use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct NbcNews;

impl crate::source::Source for NbcNews {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://feeds.nbcnews.com/nbcnews/public/world",
            ),
            (
                EndpointScope::US,
                "https://feeds.nbcnews.com/nbcnews/public/us-news",
            ),
            (
                EndpointScope::Politics,
                "https://feeds.nbcnews.com/nbcnews/public/politics",
            ),
            (
                EndpointScope::Business,
                "https://feeds.nbcnews.com/nbcnews/public/business",
            ),
            (
                EndpointScope::Technology,
                "https://feeds.nbcnews.com/nbcnews/public/tech",
            ),
            (
                EndpointScope::Entertainment,
                "https://feeds.nbcnews.com/nbcnews/public/pop-culture",
            ),
            (
                EndpointScope::Science,
                "https://feeds.nbcnews.com/nbcnews/public/science",
            ),
            (
                EndpointScope::Health,
                "https://feeds.nbcnews.com/nbcnews/public/health",
            ),
        ])
    }
}
