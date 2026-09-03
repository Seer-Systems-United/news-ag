use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Polygon,
    904,
    EndpointScope::Entertainment,
    "https://www.polygon.com/rss/index.xml"
);
