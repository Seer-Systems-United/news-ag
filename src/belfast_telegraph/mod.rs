use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    BelfastTelegraph,
    EndpointScope::World,
    "https://www.belfasttelegraph.co.uk/rss"
);
