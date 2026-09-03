use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    BusinessInsider,
    105,
    EndpointScope::Business,
    "https://feeds.businessinsider.com/custom/all"
);
