use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    ProvidenceJournal,
    EndpointScope::US,
    "https://www.providencejournal.com/news-sitemap.xml"
);
