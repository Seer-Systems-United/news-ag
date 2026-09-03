use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Newsweek,
    830,
    EndpointScope::US,
    "https://www.newsweek.com/rss"
);
