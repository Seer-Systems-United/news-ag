use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Nature,
    EndpointScope::Science,
    "https://www.nature.com/nature.rss"
);
