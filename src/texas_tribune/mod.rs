use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(TexasTribune, 914, EndpointScope::Politics, "https://www.texastribune.org/feed/");
