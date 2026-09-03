use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NikkeiAsia,
    604,
    EndpointScope::World,
    "https://asia.nikkei.com/rss/feed/nar"
);
