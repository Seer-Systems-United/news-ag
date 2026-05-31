use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Time, EndpointScope::US, "https://time.com/feed/");
