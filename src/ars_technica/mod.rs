use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ArsTechnica,
    EndpointScope::Technology,
    "https://feeds.arstechnica.com/arstechnica/index"
);
