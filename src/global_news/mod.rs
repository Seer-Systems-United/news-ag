use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    GlobalNews,
    413,
    EndpointScope::World,
    "https://globalnews.ca/feed/"
);
