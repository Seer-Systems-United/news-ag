use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Zeteo,
    837,
    EndpointScope::Politics,
    "https://zeteo.com/feed"
);
