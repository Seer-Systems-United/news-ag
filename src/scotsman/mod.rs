use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Scotsman,
    EndpointScope::World,
    "https://www.scotsman.com/rss"
);
