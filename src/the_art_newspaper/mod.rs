use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TheArtNewspaper,
    EndpointScope::Entertainment,
    "https://www.theartnewspaper.com/rss.xml"
);
