use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DerSpiegel,
    702,
    EndpointScope::World,
    "https://www.spiegel.de/international/index.rss"
);
