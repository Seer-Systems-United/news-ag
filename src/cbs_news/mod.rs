use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    CbsNews,
    EndpointScope::US,
    "https://www.cbsnews.com/latest/rss/main"
);
