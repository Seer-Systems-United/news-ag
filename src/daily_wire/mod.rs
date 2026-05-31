use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DailyWire,
    EndpointScope::Politics,
    "https://www.dailywire.com/feeds/rss.xml"
);
