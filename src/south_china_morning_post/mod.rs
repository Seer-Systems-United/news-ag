use crate::source::{
    endpoint::{Endpoint, EndpointScope},
    feed,
};

pub struct SouthChinaMorningPost;

impl crate::source::Source for SouthChinaMorningPost {
    fn endpoints() -> Vec<Endpoint> {
        feed::rss_endpoints(&[
            (EndpointScope::World, "https://www.scmp.com/rss/5/feed"),
            (EndpointScope::US, "https://www.scmp.com/rss/322262/feed"),
            (EndpointScope::Business, "https://www.scmp.com/rss/92/feed"),
            (
                EndpointScope::Technology,
                "https://www.scmp.com/rss/36/feed",
            ),
            (
                EndpointScope::Entertainment,
                "https://www.scmp.com/rss/23/feed",
            ),
            (EndpointScope::Sports, "https://www.scmp.com/rss/95/feed"),
            (
                EndpointScope::Science,
                "https://www.scmp.com/rss/318224/feed",
            ),
        ])
    }
}
