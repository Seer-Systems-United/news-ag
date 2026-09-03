use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Nme,
    913,
    EndpointScope::Entertainment,
    "https://www.nme.com/feed"
);
