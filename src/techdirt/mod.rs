use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Techdirt, 928, EndpointScope::Technology, "https://www.techdirt.com/feed");
