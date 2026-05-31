use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Pcmag,
    EndpointScope::Technology,
    "https://www.pcmag.com/feeds/rss/latest"
);
