use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    JerusalemPost,
    611,
    EndpointScope::World,
    "https://www.jpost.com/rss/rssfeedsfrontpage.aspx"
);
