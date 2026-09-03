use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Techmeme, 930, EndpointScope::Technology, "https://www.techmeme.com/feed.xml");
