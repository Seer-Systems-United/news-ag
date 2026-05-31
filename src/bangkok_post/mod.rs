use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    BangkokPost,
    EndpointScope::World,
    "https://www.bangkokpost.com/rss/data/topstories.xml"
);
