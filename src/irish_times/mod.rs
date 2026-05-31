use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    IrishTimes,
    EndpointScope::World,
    "https://www.irishtimes.com/cmlink/news-1.1319192"
);
