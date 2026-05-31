use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ScientificAmerican,
    EndpointScope::Science,
    "https://www.scientificamerican.com/platform/syndication/rss/"
);
