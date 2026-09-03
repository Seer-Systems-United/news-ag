use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(RFI, 946, EndpointScope::World, "https://www.rfi.fr/en/rss");
