use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FastCompany,
    EndpointScope::Business,
    "https://www.fastcompany.com/rss"
);
