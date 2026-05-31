use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Nme,
    EndpointScope::Entertainment,
    "https://www.nme.com/feed"
);
