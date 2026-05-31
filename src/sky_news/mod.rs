use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    SkyNews,
    EndpointScope::World,
    "https://feeds.skynews.com/feeds/rss/home.xml"
);
