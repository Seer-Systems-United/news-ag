use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    LaPresse,
    415,
    EndpointScope::World,
    "https://www.lapresse.ca/actualites/rss"
);
