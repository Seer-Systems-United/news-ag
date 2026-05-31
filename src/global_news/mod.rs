use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    GlobalNews,
    EndpointScope::World,
    "https://globalnews.ca/feed/"
);
