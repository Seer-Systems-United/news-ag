use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    CommonDreams,
    EndpointScope::Politics,
    "https://www.commondreams.org/feeds/feed.rss"
);
