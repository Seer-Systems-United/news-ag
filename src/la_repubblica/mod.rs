use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    LaRepubblica,
    EndpointScope::World,
    "https://www.repubblica.it/rss/homepage/rss2.0.xml"
);
