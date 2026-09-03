use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    MarketWatch,
    109,
    EndpointScope::Business,
    "https://feeds.marketwatch.com/marketwatch/topstories/"
);
