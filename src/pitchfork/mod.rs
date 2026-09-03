use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Pitchfork,
    907,
    EndpointScope::Entertainment,
    "https://pitchfork.com/feed/feed-news/rss"
);
