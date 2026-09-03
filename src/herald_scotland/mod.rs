use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    HeraldScotland,
    509,
    EndpointScope::World,
    "https://www.heraldscotland.com/news/rss/"
);
