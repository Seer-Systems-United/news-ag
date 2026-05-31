use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DeutscheWelle,
    EndpointScope::World,
    "https://rss.dw.com/rdf/rss-en-all"
);
