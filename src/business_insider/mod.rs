use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    BusinessInsider,
    EndpointScope::Business,
    "https://feeds.businessinsider.com/custom/all"
);
