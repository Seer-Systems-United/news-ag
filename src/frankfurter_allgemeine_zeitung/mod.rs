use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    FrankfurterAllgemeineZeitung,
    708,
    EndpointScope::World,
    "https://www.faz.net/rss/aktuell/"
);
