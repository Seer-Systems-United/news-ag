use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ViceNews,
    814,
    EndpointScope::US,
    "https://www.vice.com/en/rss"
);
