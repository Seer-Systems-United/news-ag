use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ForeignPolicy,
    EndpointScope::World,
    "https://foreignpolicy.com/feed/"
);
