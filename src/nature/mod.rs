use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Nature,
    207,
    EndpointScope::Science,
    "https://www.nature.com/nature.rss"
);
