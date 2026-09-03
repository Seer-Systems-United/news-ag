use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    RollingStone,
    902,
    EndpointScope::Entertainment,
    "https://www.rollingstone.com/feed/"
);
