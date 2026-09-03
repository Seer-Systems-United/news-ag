use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Empire,
    908,
    EndpointScope::Entertainment,
    "https://rss.onebauer.media/api/feed-aggregator?hostname=https://www.empireonline.com"
);
