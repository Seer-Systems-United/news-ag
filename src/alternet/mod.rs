use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Alternet, 917, EndpointScope::Politics, "https://www.alternet.org/feed");
