use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NewStatesman,
    EndpointScope::Politics,
    "https://www.newstatesman.com/feed"
);
