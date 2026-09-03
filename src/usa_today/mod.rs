use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    UsaToday,
    9,
    EndpointScope::US,
    "https://www.usatoday.com/news-sitemap.xml"
);
