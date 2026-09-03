use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(NikkeiEnglish, 921, EndpointScope::World, "https://asia.nikkei.com/rss/feed/nar");
