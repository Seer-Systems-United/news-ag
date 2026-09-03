use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Salon, 916, EndpointScope::Politics, "https://www.salon.com/feed/");
