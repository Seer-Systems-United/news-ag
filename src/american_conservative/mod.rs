use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    AmericanConservative,
    838,
    EndpointScope::Politics,
    "https://www.theamericanconservative.com/feed/"
);
