use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NbcNews,
    EndpointScope::US,
    "https://feeds.nbcnews.com/nbcnews/public/news"
);
