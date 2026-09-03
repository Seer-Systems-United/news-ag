use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    DropSiteNews,
    836,
    EndpointScope::Politics,
    "https://www.dropsitenews.com/feed"
);
