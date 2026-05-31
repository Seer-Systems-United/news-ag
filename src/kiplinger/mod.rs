use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Kiplinger,
    EndpointScope::Business,
    "https://www.kiplinger.com/investing/feed"
);
