use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(TomsGuide, 925, EndpointScope::Technology, "https://www.tomsguide.com/feeds/all");
