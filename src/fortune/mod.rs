use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Fortune,
    EndpointScope::Business,
    "https://fortune.com/feed/"
);
