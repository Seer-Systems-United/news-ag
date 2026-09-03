use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Gizmodo,
    205,
    EndpointScope::Technology,
    "https://gizmodo.com/rss"
);
