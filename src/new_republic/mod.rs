use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NewRepublic,
    817,
    EndpointScope::Politics,
    "https://newrepublic.com/rss.xml"
);
