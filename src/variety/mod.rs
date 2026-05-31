use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Variety,
    EndpointScope::Entertainment,
    "https://variety.com/feed/"
);
