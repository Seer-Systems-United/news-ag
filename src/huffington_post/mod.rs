use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    HuffingtonPost,
    EndpointScope::US,
    "https://www.huffpost.com/static-assets/isolated/huffpostsitemapgeneratorjob-prod-public/us/sitemaps/sitemap-google-news.xml"
);
