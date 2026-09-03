use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Jacobin,
    828,
    EndpointScope::Politics,
    "https://jacobin.com/rss"
);
