use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    CorriereDellaSera,
    707,
    EndpointScope::World,
    "https://www.corriere.it/rss/homepage.xml"
);
