use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ForeignPolicy,
    832,
    EndpointScope::World,
    "https://foreignpolicy.com/feed/"
);
