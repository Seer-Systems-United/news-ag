use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MarketWatch,
    EndpointScope::Business,
    "https://feeds.marketwatch.com/marketwatch/topstories/"
);
