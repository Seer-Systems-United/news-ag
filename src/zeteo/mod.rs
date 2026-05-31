use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Zeteo, EndpointScope::Politics, "https://zeteo.com/feed");
