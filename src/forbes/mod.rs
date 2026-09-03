use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Forbes,
    107,
    EndpointScope::Business,
    "https://www.forbes.com/most-popular/feed/"
);
