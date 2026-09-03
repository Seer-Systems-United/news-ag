use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(ArabNews, 952, EndpointScope::World, "https://www.arabnews.com/rss.xml");
