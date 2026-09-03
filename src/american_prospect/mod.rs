use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    AmericanProspect,
    819,
    EndpointScope::Politics,
    "https://prospect.org/feed/"
);
