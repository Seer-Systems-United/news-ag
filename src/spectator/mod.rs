use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    Spectator,
    821,
    EndpointScope::World,
    "https://spectator.com/news-sitemap.xml"
);
