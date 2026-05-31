use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    GameSpot,
    EndpointScope::Entertainment,
    "https://www.gamespot.com/feeds/mashup/"
);
