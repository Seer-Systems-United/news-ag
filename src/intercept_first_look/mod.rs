use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(InterceptFirstLook, 920, EndpointScope::Politics, "https://theintercept.com/feed/?podcast=first-look");
