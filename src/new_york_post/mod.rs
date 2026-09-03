use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NewYorkPost,
    822,
    EndpointScope::US,
    "https://nypost.com/feed/"
);
