use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DailyKos,
    813,
    EndpointScope::Politics,
    "https://www.dailykos.com/blogs/main.rss"
);
