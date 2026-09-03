use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Kiplinger,
    112,
    EndpointScope::Business,
    "https://www.kiplinger.com/investing/feed"
);
