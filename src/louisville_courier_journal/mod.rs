use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    LouisvilleCourierJournal,
    EndpointScope::US,
    "https://www.courier-journal.com/news-sitemap.xml"
);
