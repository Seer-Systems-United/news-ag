use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Msnbc, EndpointScope::US, "https://www.ms.now/feed/");
