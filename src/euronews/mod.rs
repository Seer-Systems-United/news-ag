use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Euronews,
    EndpointScope::World,
    "https://www.euronews.com/rss?level=theme&name=news"
);
