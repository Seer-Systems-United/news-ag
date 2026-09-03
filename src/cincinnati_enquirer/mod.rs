use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    CincinnatiEnquirer,
    318,
    EndpointScope::US,
    "https://www.cincinnati.com/news-sitemap.xml"
);
