use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Nation,
    818,
    EndpointScope::Politics,
    "https://www.thenation.com/feed/?post_type=article"
);
