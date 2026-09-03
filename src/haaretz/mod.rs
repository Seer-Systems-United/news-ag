use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Haaretz,
    610,
    EndpointScope::World,
    "https://www.haaretz.com/srv/haaretz-latest-headlines"
);
