use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Truthout, 919, EndpointScope::Politics, "https://truthout.org/feed/");
