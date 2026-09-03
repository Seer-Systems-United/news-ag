use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(NerdWallet, 940, EndpointScope::Business, "https://www.nerdwallet.com/blog/feed/");
