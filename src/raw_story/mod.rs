use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(RawStory, 918, EndpointScope::Politics, "https://www.rawstory.com/feed/");
