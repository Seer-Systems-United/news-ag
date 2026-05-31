use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    AbcNews,
    EndpointScope::US,
    "https://abcnews.go.com/abcnews/topstories"
);
