use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MiddleEastEye,
    612,
    EndpointScope::World,
    "https://www.middleeasteye.net/rss.xml"
);
