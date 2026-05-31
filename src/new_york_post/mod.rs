use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(NewYorkPost, EndpointScope::US, "https://nypost.com/feed/");
