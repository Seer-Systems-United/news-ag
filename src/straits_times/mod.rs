use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    StraitsTimes,
    EndpointScope::World,
    "https://www.straitstimes.com/news/world/rss.xml"
);
