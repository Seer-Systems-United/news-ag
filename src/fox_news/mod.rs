use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FoxNews,
    EndpointScope::US,
    "https://moxie.foxnews.com/google-publisher/latest.xml"
);
