use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NikkeiAsia,
    EndpointScope::World,
    "https://asia.nikkei.com/rss/feed/nar"
);
