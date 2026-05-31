use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    SouthChinaMorningPost,
    EndpointScope::World,
    "https://www.scmp.com/rss/91/feed"
);
