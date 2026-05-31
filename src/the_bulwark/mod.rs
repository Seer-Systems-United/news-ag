use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TheBulwark,
    EndpointScope::Politics,
    "https://www.thebulwark.com/feed/"
);
