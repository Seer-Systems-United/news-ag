use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    NashvilleTennessean,
    EndpointScope::US,
    "https://www.tennessean.com/news-sitemap.xml"
);
