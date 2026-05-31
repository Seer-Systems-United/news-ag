use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ScienceMagazine,
    EndpointScope::Science,
    "https://www.science.org/rss/news_current.xml"
);
