use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    IrishTimes,
    508,
    EndpointScope::World,
    "https://www.irishtimes.com/cmlink/news-1.1319192"
);
