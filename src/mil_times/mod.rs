use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(MilitaryTimes, 932, EndpointScope::Politics, "https://www.militarytimes.com/arc/outboundfeeds/rss/");
