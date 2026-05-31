use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    TheHindu,
    EndpointScope::World,
    "https://www.thehindu.com/feeder/default.rss"
);
