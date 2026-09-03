use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Variety,
    900,
    EndpointScope::Entertainment,
    "https://variety.com/feed/"
);
