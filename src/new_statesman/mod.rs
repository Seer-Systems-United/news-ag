use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NewStatesman,
    820,
    EndpointScope::Politics,
    "https://www.newstatesman.com/feed"
);
