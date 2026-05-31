use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    Spectator,
    EndpointScope::World,
    "https://spectator.com/news-sitemap.xml"
);
