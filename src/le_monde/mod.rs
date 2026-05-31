use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    LeMonde,
    EndpointScope::World,
    "https://www.lemonde.fr/en/rss/une.xml"
);
