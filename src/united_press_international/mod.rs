use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    UnitedPressInternational,
    16,
    EndpointScope::World,
    "https://www.upi.com/rss/news.rss"
);
