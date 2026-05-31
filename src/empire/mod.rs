use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Empire,
    EndpointScope::Entertainment,
    "https://rss.onebauer.media/api/feed-aggregator?hostname=https://www.empireonline.com"
);
