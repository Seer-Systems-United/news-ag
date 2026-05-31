use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    VentureBeat,
    EndpointScope::Technology,
    "https://venturebeat.com/feed/"
);
