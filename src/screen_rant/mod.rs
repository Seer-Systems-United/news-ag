use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    ScreenRant,
    912,
    EndpointScope::Entertainment,
    "https://screenrant.com/feed/"
);
