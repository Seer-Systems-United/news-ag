use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MotherJones,
    825,
    EndpointScope::Politics,
    "https://www.motherjones.com/feed/"
);
