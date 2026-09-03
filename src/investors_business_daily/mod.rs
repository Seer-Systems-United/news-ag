use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(InvestorsBusinessDaily, 936, EndpointScope::Business, "https://www.investors.com/feed/");
