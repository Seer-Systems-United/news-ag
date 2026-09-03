use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    VentureBeat,
    213,
    EndpointScope::Technology,
    "https://venturebeat.com/feed/"
);
