use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Wired,
    200,
    EndpointScope::Technology,
    "https://www.wired.com/feed/rss"
);
