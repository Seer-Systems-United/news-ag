use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Wired,
    EndpointScope::Technology,
    "https://www.wired.com/feed/rss"
);
