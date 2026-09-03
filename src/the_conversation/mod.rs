use crate::source::{endpoint::EndpointScope, feed};

feed::rss_source!(Conversation, 915, EndpointScope::World, "https://theconversation.com/articles.atom");
