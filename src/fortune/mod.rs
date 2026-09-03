use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Fortune,
    106,
    EndpointScope::Business,
    "https://fortune.com/feed/"
);
