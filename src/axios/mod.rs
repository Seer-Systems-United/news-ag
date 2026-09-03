use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Axios,
    816,
    EndpointScope::US,
    "https://www.axios.com/feeds/feed.rss"
);
