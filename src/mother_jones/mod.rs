use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MotherJones,
    EndpointScope::Politics,
    "https://www.motherjones.com/feed/"
);
