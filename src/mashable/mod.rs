use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Mashable,
    214,
    EndpointScope::Technology,
    "https://mashable.com/feeds/rss/all"
);
