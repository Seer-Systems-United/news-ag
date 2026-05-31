use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    NewRepublic,
    EndpointScope::Politics,
    "https://newrepublic.com/rss.xml"
);
