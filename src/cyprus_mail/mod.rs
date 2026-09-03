use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(CyprusMail, 954, EndpointScope::World, "https://cyprus-mail.com/feed");
