use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(GlobalVoices, 957, EndpointScope::World, "https://globalvoices.org/feed/");
