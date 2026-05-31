use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Mashable,
    EndpointScope::Technology,
    "https://mashable.com/feeds/rss/all"
);
