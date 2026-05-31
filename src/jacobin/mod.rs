use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Jacobin, EndpointScope::Politics, "https://jacobin.com/rss");
