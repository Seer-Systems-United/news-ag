use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(
    StarLedger,
    EndpointScope::US,
    "https://www.nj.com/arc/outboundfeeds/rss/?outputType=xml"
);
