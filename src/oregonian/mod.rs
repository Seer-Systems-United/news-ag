use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    Oregonian,
    EndpointScope::US,
    "https://www.oregonlive.com/arc/outboundfeeds/rss/?outputType=xml"
);
