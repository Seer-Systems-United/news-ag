use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ProPublica,
    EndpointScope::US,
    "https://feeds.propublica.org/propublica/main"
);
