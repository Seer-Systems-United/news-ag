use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Newsweek, EndpointScope::US, "https://www.newsweek.com/rss");
