use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Clarin,
    EndpointScope::World,
    "https://www.clarin.com/rss/lo-ultimo/"
);
