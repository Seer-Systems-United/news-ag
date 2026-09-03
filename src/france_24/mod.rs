use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    France24,
    11,
    EndpointScope::World,
    "https://www.france24.com/en/rss"
);
