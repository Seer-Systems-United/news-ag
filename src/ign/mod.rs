use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Ign,
    EndpointScope::Entertainment,
    "https://feeds.feedburner.com/ign/all"
);
