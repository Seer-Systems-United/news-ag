use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Quartz, EndpointScope::Business, "https://qz.com/rss");
