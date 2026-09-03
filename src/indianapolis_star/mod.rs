use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    IndianapolisStar,
    323,
    EndpointScope::US,
    "https://www.indystar.com/news-sitemap.xml"
);
