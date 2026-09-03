use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Ign,
    905,
    EndpointScope::Entertainment,
    "https://feeds.feedburner.com/ign/all"
);
