use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DerSpiegel,
    EndpointScope::World,
    "https://www.spiegel.de/international/index.rss"
);
