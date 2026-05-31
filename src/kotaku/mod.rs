use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Kotaku,
    EndpointScope::Entertainment,
    "https://kotaku.com/rss"
);
