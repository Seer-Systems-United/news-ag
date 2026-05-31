use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DropSiteNews,
    EndpointScope::Politics,
    "https://www.dropsitenews.com/feed"
);
