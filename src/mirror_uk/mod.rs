use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(MirrorUK, 943, EndpointScope::World, "https://www.mirror.co.uk/news/?service=rss");
