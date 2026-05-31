use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    UsaToday,
    EndpointScope::US,
    "https://www.usatoday.com/news-sitemap.xml"
);
