use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Gizmodo,
    EndpointScope::Technology,
    "https://gizmodo.com/rss"
);
