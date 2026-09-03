use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ProPublica,
    827,
    EndpointScope::US,
    "https://feeds.propublica.org/propublica/main"
);
