use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Billboard,
    906,
    EndpointScope::Entertainment,
    "https://www.billboard.com/feed/"
);
