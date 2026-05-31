use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Nation,
    EndpointScope::Politics,
    "https://www.thenation.com/feed/?post_type=article"
);
