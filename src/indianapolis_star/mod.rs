use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    IndianapolisStar,
    EndpointScope::US,
    "https://www.indystar.com/news-sitemap.xml"
);
