use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(TelegraphUK, 944, EndpointScope::World, "https://www.telegraph.co.uk/rss.xml");
