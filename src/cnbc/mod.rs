use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Cnbc,
    EndpointScope::Business,
    "https://search.cnbc.com/rs/search/combinedcms/view.xml?partnerId=wrss01&id=100003114"
);
