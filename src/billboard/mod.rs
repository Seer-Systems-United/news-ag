use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Billboard,
    EndpointScope::Entertainment,
    "https://www.billboard.com/feed/"
);
