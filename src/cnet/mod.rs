use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Cnet,
    EndpointScope::Technology,
    "https://www.cnet.com/rss/news/"
);
