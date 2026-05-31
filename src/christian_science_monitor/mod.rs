use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ChristianScienceMonitor,
    EndpointScope::US,
    "https://rss.csmonitor.com/feeds/usa"
);
