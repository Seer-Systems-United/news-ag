use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Msnbc, 823, EndpointScope::US, "https://www.ms.now/feed/");
