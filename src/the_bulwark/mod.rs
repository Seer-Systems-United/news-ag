use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TheBulwark,
    834,
    EndpointScope::Politics,
    "https://www.thebulwark.com/feed/"
);
