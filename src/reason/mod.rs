use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Reason,
    826,
    EndpointScope::Politics,
    "https://reason.com/feed/"
);
