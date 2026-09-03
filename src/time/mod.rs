use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Time, 831, EndpointScope::US, "https://time.com/feed/");
