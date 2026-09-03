use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Kotaku,
    903,
    EndpointScope::Entertainment,
    "https://kotaku.com/rss"
);
