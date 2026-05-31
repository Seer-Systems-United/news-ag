use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    OklahomaCityOklahoman,
    EndpointScope::US,
    "https://www.oklahoman.com/news-sitemap.xml"
);
