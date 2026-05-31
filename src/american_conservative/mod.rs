use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    AmericanConservative,
    EndpointScope::Politics,
    "https://www.theamericanconservative.com/feed/"
);
