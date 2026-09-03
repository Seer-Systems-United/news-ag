use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FastCompany,
    111,
    EndpointScope::Business,
    "https://www.fastcompany.com/rss"
);
