use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    DetroitFreePress,
    EndpointScope::US,
    "https://www.freep.com/news-sitemap.xml"
);
