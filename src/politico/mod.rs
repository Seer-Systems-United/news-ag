use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Politico,
    EndpointScope::Politics,
    "https://rss.politico.com/politics-news.xml"
);
