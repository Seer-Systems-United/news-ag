use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DailyWire,
    812,
    EndpointScope::Politics,
    "https://www.dailywire.com/feeds/rss.xml"
);
