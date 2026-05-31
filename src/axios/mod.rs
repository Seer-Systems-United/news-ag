use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Axios,
    EndpointScope::US,
    "https://www.axios.com/feeds/feed.rss"
);
