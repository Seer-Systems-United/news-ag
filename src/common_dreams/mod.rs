use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    CommonDreams,
    835,
    EndpointScope::Politics,
    "https://www.commondreams.org/feeds/feed.rss"
);
