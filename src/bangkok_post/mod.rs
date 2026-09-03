use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct BangkokPost;

impl crate::source::Source for BangkokPost {
    fn id() -> uuid::Uuid {
        crate::source::feed::source_id(614)
    }
    fn name() -> &'static str {
        crate::source::feed::source_name(614)
    }
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (
                EndpointScope::World,
                "https://www.bangkokpost.com/rss/data/world.xml",
            ),
            (
                EndpointScope::Business,
                "https://www.bangkokpost.com/rss/data/business.xml",
            ),
            (
                EndpointScope::Technology,
                "https://www.bangkokpost.com/rss/data/tech.xml",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.bangkokpost.com/rss/data/life.xml",
            ),
            (
                EndpointScope::Sports,
                "https://www.bangkokpost.com/rss/data/sports.xml",
            ),
        ])
    }
}
