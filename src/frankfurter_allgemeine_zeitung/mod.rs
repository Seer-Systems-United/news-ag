use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FrankfurterAllgemeineZeitung,
    EndpointScope::World,
    "https://www.faz.net/rss/aktuell/"
);
