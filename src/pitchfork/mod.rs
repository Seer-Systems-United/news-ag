use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Pitchfork,
    EndpointScope::Entertainment,
    "https://pitchfork.com/feed/feed-news/rss"
);
