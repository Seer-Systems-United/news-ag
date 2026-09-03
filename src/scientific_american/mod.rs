use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ScientificAmerican,
    208,
    EndpointScope::Science,
    "https://www.scientificamerican.com/platform/syndication/rss/"
);
