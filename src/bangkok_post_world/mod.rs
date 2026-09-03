use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(BangkokPostWorld, 922, EndpointScope::World, "https://www.bangkokpost.com/rss/data/topstories.xml");
