use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(NHKWorld, 947, EndpointScope::World, "https://www3.nhk.or.jp/rss/news/cat0.xml");
