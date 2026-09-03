use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    BelfastTelegraph,
    510,
    EndpointScope::World,
    "https://www.belfasttelegraph.co.uk/rss"
);
