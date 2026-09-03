use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    AlMonitor,
    18,
    EndpointScope::World,
    "https://www.al-monitor.com/rss"
);
