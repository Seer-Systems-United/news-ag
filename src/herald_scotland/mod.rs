use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    HeraldScotland,
    EndpointScope::World,
    "https://www.heraldscotland.com/news/rss/"
);
