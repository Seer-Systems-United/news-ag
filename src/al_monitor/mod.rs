use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    AlMonitor,
    EndpointScope::World,
    "https://www.al-monitor.com/rss"
);
