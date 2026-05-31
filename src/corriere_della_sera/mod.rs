use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    CorriereDellaSera,
    EndpointScope::World,
    "https://www.corriere.it/rss/homepage.xml"
);
