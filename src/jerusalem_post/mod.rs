use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    JerusalemPost,
    EndpointScope::World,
    "https://www.jpost.com/rss/rssfeedsfrontpage.aspx"
);
