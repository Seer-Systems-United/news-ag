use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    JapanTimes,
    EndpointScope::World,
    "https://www.japantimes.co.jp/feed/"
);
