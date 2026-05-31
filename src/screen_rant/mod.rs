use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ScreenRant,
    EndpointScope::Entertainment,
    "https://screenrant.com/feed/"
);
