use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DailyKos,
    EndpointScope::Politics,
    "https://www.dailykos.com/blogs/main.rss"
);
