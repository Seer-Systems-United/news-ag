use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Scotsman,
    511,
    EndpointScope::World,
    "https://www.scotsman.com/rss"
);
