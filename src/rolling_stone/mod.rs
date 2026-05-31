use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    RollingStone,
    EndpointScope::Entertainment,
    "https://www.rollingstone.com/feed/"
);
