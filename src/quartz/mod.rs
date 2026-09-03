use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Quartz, 108, EndpointScope::Business, "https://qz.com/rss");
