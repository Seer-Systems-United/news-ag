use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MiddleEastEye,
    EndpointScope::World,
    "https://www.middleeasteye.net/rss.xml"
);
