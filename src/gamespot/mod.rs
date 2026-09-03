use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    GameSpot,
    910,
    EndpointScope::Entertainment,
    "https://www.gamespot.com/feeds/mashup/"
);
