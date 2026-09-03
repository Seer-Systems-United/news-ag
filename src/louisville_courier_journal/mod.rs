use crate::source::{endpoint::EndpointScope, feed};

feed::news_sitemap_source!(
    LouisvilleCourierJournal,
    324,
    EndpointScope::US,
    "https://www.courier-journal.com/news-sitemap.xml"
);
