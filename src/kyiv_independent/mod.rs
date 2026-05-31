use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    KyivIndependent,
    EndpointScope::World,
    "https://kyivindependent.com/news-archive/rss/"
);
