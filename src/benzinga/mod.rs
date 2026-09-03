use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Benzinga, 939, EndpointScope::Business, "https://www.benzinga.com/feed");
