use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(ViceNews, EndpointScope::US, "https://www.vice.com/en/rss");
