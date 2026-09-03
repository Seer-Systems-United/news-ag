use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TheArtNewspaper,
    909,
    EndpointScope::Entertainment,
    "https://www.theartnewspaper.com/rss.xml"
);
