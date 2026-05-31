use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    AmericanProspect,
    EndpointScope::Politics,
    "https://prospect.org/feed/"
);
