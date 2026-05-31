use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    LaPresse,
    EndpointScope::World,
    "https://www.lapresse.ca/actualites/rss"
);
