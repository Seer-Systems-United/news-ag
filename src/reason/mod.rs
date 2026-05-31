use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Reason, EndpointScope::Politics, "https://reason.com/feed/");
